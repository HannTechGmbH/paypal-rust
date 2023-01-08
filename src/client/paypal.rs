use std::sync::Arc;

use crate::client::request;

use crate::client::app_info::AppInfo;
use crate::client::auth::{AuthData, AuthResponse, AuthStrategy, Authenticate};
use crate::client::endpoint::Endpoint;
use crate::client::error::{PayPalError, ValidationError};
use crate::client::request::QueryParams;
use http_types::Url;
use reqwest::header::AUTHORIZATION;
use reqwest::RequestBuilder;
use reqwest_middleware;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub static USER_AGENT: &str = concat!("PayPal/v2 Rust Bindings/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct Client {
    pub default_headers: request::HttpRequestHeaders,
    pub auth_data: Arc<RwLock<AuthData>>,
    user_agent: String,
    client_secret: String,
    username: String,
    environment: Environment,
    base_url: Url,
    http: reqwest::Client,
}

impl Client {
    /// Initialize a new PayPal client. To authenticate, use the `authenticate` method.
    pub fn new(
        username: String,
        client_secret: String,
        environment: Environment,
    ) -> Result<Self, Box<PayPalError>> {
        let authorization =
            get_basic_auth_for_user_service(username.as_str(), client_secret.as_str());

        let base_url = match environment {
            Environment::Sandbox => request::RequestUrl::Sandbox,
            Environment::Live => request::RequestUrl::Live,
        }
        .as_url()
        .map_err(|_e| PayPalError::LibraryError("Could not parse environment Url".to_string()))?;

        Ok(Client {
            environment,
            client_secret,
            username,
            default_headers: request::HttpRequestHeaders::new(authorization),
            base_url,
            http: reqwest::Client::new(),
            user_agent: USER_AGENT.into(),
            auth_data: Arc::new(RwLock::new(AuthData::default())),
        })
    }

    /// Composes an URL from the base URL and the provided path.
    ///
    /// # Arguments
    ///  * `request_path` - The path to append to the base URL.
    pub fn compose_url(&self, request_path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(request_path);
        url
    }

    /// Composes an URL with query parameters.
    ///
    /// # Arguments
    /// * `request_path` - The path to append to the base URL.
    /// * `query` - The query parameters to append to the URL.
    pub fn compose_url_with_query(
        &self,
        request_path: &str,
        query: &QueryParams,
    ) -> Result<Url, serde_qs::Error> {
        let mut url = self.compose_url(request_path);
        let params = serde_qs::to_string(query)?;

        if params.is_empty() {
            return Ok(url);
        }

        url.set_query(Some(&params));
        Ok(url)
    }

    pub fn with_app_info(mut self, app_info: AppInfo) -> Self {
        self.user_agent = format!("{} {}", self.user_agent, app_info.to_string());
        self
    }

    /// Performs a GET request.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint to call.
    ///
    /// # Returns
    /// The response body serialized into the provided type.
    pub async fn get<T: Endpoint>(&self, endpoint: &T) -> Result<T::ResponseBody, PayPalError> {
        let mut req = self.http.get(endpoint.request_url(self.environment));
        req = self.set_request_headers(req, &endpoint.headers());

        let response = self.execute(endpoint, req).await?;

        Ok(response)
    }

    /// Performs a POST request.
    /// # Arguments
    /// * `endpoint` - The endpoint to call.
    ///
    /// # Returns
    /// The response body serialized into the provided type.
    pub async fn post<T: Endpoint>(&self, endpoint: &T) -> Result<T::ResponseBody, PayPalError> {
        let body = serde_json::to_string(&endpoint.request_body())?;
        let mut req = self.http.post(endpoint.request_url(self.environment));

        req = self.set_request_headers(req, &endpoint.headers());
        let response = self.execute(endpoint, req.body(body)).await?;

        Ok(response)
    }

    /// Performs a PATCH request.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint to call.
    ///
    /// # Returns
    /// The response body serialized into the provided type.
    pub async fn patch<T: Endpoint>(&self, endpoint: &T) -> Result<(), PayPalError> {
        let body = serde_json::to_string(&endpoint.request_body())?;
        let mut req = self.http.patch(endpoint.request_url(self.environment));

        req = self.set_request_headers(req, &endpoint.headers());
        self.execute(endpoint, req.body(body)).await?;

        Ok(())
    }

    /// Sets the request headers for a request.
    ///
    /// # Arguments
    /// * `request_builder` - The request builder to set the headers on.
    /// * `headers` - The headers to set.
    pub fn set_request_headers(
        &self,
        mut request_builder: RequestBuilder,
        headers: &request::HttpRequestHeaders,
    ) -> RequestBuilder {
        for (key, value) in headers.to_vec() {
            request_builder = request_builder.header(key, value);
        }

        request_builder
    }

    /// Executes a request.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint to call.
    /// * `request` - The request to execute (builder).
    async fn execute<T: Endpoint>(
        &self,
        endpoint: &T,
        mut request: RequestBuilder,
    ) -> Result<T::ResponseBody, PayPalError> {
        if let AuthStrategy::TokenRefresh = endpoint.auth_strategy() {
            if self.auth_data.read().await.about_to_expire() {
                self.authenticate().await?;
            }
        }

        request = request.header(
            AUTHORIZATION,
            format!("Bearer {}", self.auth_data.read().await.access_token),
        );

        let response = request.send().await.map_err(PayPalError::from)?;

        if !response.status().is_success() {
            return Err(PayPalError::from(
                response
                    .json::<ValidationError>()
                    .await
                    .map_err(PayPalError::from)?,
            ));
        }

        serde_json::from_str::<T::ResponseBody>(&response.text().await.map_err(PayPalError::from)?)
            .or_else(|error| {
                // Endpoints that return an empty response body can safely be deserialized intonto
                // an empty struct.
                if error.is_eof() {
                    Ok(serde_json::from_str::<T::ResponseBody>("{}")?)
                } else {
                    Err(PayPalError::from(error))
                }
            })
    }

    pub async fn authenticate(&self) -> Result<(), PayPalError> {
        let endpoint = Authenticate::new(get_basic_auth_for_user_service(
            self.username.as_str(),
            self.client_secret.as_str(),
        ));

        let mut request = self
            .http
            .post(endpoint.request_url(self.environment))
            .body(serde_urlencoded::to_string(&endpoint.request_body())?);

        let mut retries = 0;
        if let Some(retry_count) = &endpoint.request_strategy().get_retry_count() {
            retries = (*retry_count).get();
        }

        request = self.set_request_headers(request, &endpoint.headers());
        request = request.header(
            AUTHORIZATION,
            get_basic_auth_for_user_service(&self.username, &self.client_secret),
        );

        let retry_client = reqwest_middleware::ClientBuilder::new(self.http.clone())
            .with(RetryTransientMiddleware::new_with_policy(
                ExponentialBackoff::builder().build_with_max_retries(retries),
            ))
            .build();

        let retry_request = retry_client.execute(request.build()?).await;
        match retry_request {
            Ok(res) => {
                let parsed_response = serde_json::from_str::<AuthResponse>(&res.text().await?)?;

                self.auth_data.write().await.update(parsed_response);
                Ok(())
            }

            Err(err) => Err(PayPalError::from(err)),
        }
    }
}

fn get_basic_auth_for_user_service(username: &str, client_secret: &str) -> String {
    format!(
        "Basic {}",
        base64::encode(format!("{}:{}", username, client_secret))
    )
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Live,
}

impl Environment {
    pub fn as_str(self) -> &'static str {
        match self {
            Environment::Sandbox => "sandbox",
            Environment::Live => "live",
        }
    }
}

impl AsRef<str> for Environment {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{Client, Environment, QueryParams};
    use http_types::Url;
    use std::str::FromStr;

    #[test]
    fn test_environment() {
        assert_eq!(Environment::Sandbox.as_str(), "sandbox");
        assert_eq!(Environment::Live.as_str(), "live");
    }

    #[test]
    fn test_compose_url() {
        let client = Client::new(
            "username".to_string(),
            "password".to_string(),
            Environment::Sandbox,
        )
        .unwrap();
        let url = client.compose_url("test");
        assert_eq!(
            url,
            Url::from_str("https://api-m.sandbox.paypal.com/test").unwrap()
        );

        let client = Client::new(
            "username".to_string(),
            "password".to_string(),
            Environment::Live,
        )
        .unwrap();
        let url = client.compose_url("test");
        assert_eq!(url, Url::from_str("https://api-m.paypal.com/test").unwrap());
    }

    #[test]
    fn test_compose_url_with_query() {
        let client = Client::new(
            "username".to_string(),
            "password".to_string(),
            Environment::Sandbox,
        )
        .unwrap();
        let query: QueryParams = QueryParams::new()
            .page(1)
            .page_size(10)
            .total_count_required(true);

        let url = client.compose_url_with_query("test", &query).unwrap();

        assert_eq!(
            url,
            Url::from_str("https://api-m.sandbox.paypal.com/test?page=1&page_size=10&total_count_required=true").unwrap()
        );
    }
}
