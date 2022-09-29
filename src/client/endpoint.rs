use crate::client::auth::AuthStrategy;
use crate::client::paypal::Environment;
use crate::client::request::{HttpRequestHeaders, RequestStrategy, RequestUrl};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

pub trait Endpoint {
    /// The query parameters the endpoint accepts.
    type QueryParams: Serialize;
    /// The request body the endpoint accepts.
    type RequestBody: Serialize;
    /// The response body the endpoint returns.
    type ResponseBody: DeserializeOwned + Debug;

    /// The path of the endpoint. Not including the base URL.
    fn path(&self) -> Cow<str>;

    /// The headers to send with the request.
    fn headers(&self) -> HttpRequestHeaders {
        HttpRequestHeaders::default()
    }

    /// The query parameters to send with the request.
    fn query(&self) -> Option<Self::QueryParams> {
        None
    }

    /// The request body to send with the request.
    fn request_body(&self) -> Option<Self::RequestBody> {
        None
    }

    /// The HTTP method to use for the request.
    fn request_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    /// The request strategy to use.
    fn request_strategy(&self) -> RequestStrategy {
        RequestStrategy::default()
    }

    /// The authorization strategy to use.
    fn auth_strategy(&self) -> AuthStrategy {
        AuthStrategy::default()
    }

    /// The URL to send the request to. DO NOT OVERRIDE THIS METHOD.
    fn request_url(&self, environment: Environment) -> Url {
        let path = self.path();
        if path.starts_with('/') {
            panic!("Path must not start with a forward slash");
        }

        let mut request_url = match environment {
            Environment::Sandbox => RequestUrl::Sandbox.as_url().expect("Invalid URL"),
            Environment::Live => RequestUrl::Live.as_url().expect("Invalid URL"),
        };

        request_url.set_path(&path);
        request_url
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Default)]
pub struct EmptyResponseBody {}
