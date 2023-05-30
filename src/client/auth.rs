use std::borrow::Cow;
use std::ops::Sub;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::endpoint::Endpoint;
use crate::client::request::{HttpRequestHeaders, RequestStrategy, RetryCount};

#[derive(Debug, Clone, Serialize)]
pub struct AuthRequestBody {
    grant_type: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct AuthResponse {
    /// Which scopes are granted. Comma seperated URLS as String.
    pub scope: Option<String>,

    /// Access token.
    pub access_token: String,

    /// Refresh token.
    pub refresh_token: Option<String>,

    /// Token type (Bearer, Basic, etc.).
    pub token_type: String,

    /// The clients App ID.
    pub app_id: String,

    /// Number of seconds until the access_token expires
    pub expires_in: i32,

    /// Authentication Nonce.
    pub nonce: String,
}

#[derive(Debug)]
pub struct Authenticate {
    pub authorization: String,
}

impl Authenticate {
    pub const fn new(authorization: String) -> Self {
        Self { authorization }
    }
}

impl Endpoint for Authenticate {
    type QueryParams = ();
    type RequestBody = AuthRequestBody;
    type ResponseBody = AuthResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v1/oauth2/token")
    }

    fn headers(&self) -> HttpRequestHeaders {
        HttpRequestHeaders {
            authorization: self.authorization.clone(),
            content_type: "application/x-www-form-urlencoded".to_string(),
            ..Default::default()
        }
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(AuthRequestBody {
            grant_type: "client_credentials".to_string(),
        })
    }

    fn request_method(&self) -> Method {
        Method::POST
    }

    fn request_strategy(&self) -> RequestStrategy {
        RequestStrategy::Retry(RetryCount::from(3))
    }

    fn auth_strategy(&self) -> AuthStrategy {
        AuthStrategy::NoTokenRefresh
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
/// Whether to check the validity of the access token and refresh it if necessary or not.
pub enum AuthStrategy {
    /// Always check the validity of the access token and refresh it if necessary.
    #[default]
    TokenRefresh,
    /// Never check the validity of the access token and never refresh it.
    NoTokenRefresh,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct AuthData {
    pub access_token: String,
    pub refresh_token: Option<String>,

    /// The timestamp the access token expires in.
    pub expiry_time: Option<DateTime<Utc>>,
}

impl AuthData {
    pub fn about_to_expire(&self) -> bool {
        self.expiry_time.map_or(true, |expiry_time| {
            expiry_time.sub(Utc::now()).num_seconds() < 10
        })
    }

    pub fn update(&mut self, response: AuthResponse) {
        self.access_token = response.access_token;
        self.refresh_token = response.refresh_token;
        self.expiry_time =
            Some(Utc::now() + chrono::Duration::seconds(i64::from(response.expires_in)));
    }
}
