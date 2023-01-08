use crate::resources::link_description::LinkDescription;
use reqwest_middleware;
use reqwest_middleware::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use std::fmt::Display;
use thiserror::Error as ThisErr;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ErrorDetails {
    pub field: Option<String>,
    pub value: Option<String>,
    pub location: Option<String>,
    pub issue: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ThisErr)]
pub struct ValidationError {
    pub name: String,
    pub message: String,
    pub debug_id: Option<String>,
    pub details: Option<Vec<ErrorDetails>>,
    pub links: Vec<LinkDescription>,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValidationError: {} - {} - {:?} - {:?}\n Links: {:?}",
            self.name, self.message, self.debug_id, self.details, self.links
        )
    }
}

pub trait ErrorResponse {
    fn error(&self) -> Cow<str>;
    fn error_description(&self) -> Cow<str>;
}

#[derive(Debug, ThisErr)]
pub enum PayPalError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api(ValidationError),
    QueryString(serde_urlencoded::ser::Error),
    MissingAccessToken,
    LibraryError(String),
}

impl Display for PayPalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {}", e),
            Self::Json(e) => write!(f, "Failed to serialize response body: {}", e),
            Self::Api(e) => write!(f, "API error: {}", e),
            Self::QueryString(e) => write!(f, "Failed to serialize query string: {}", e),
            Self::MissingAccessToken => write!(f, "Missing access token"),
            Self::LibraryError(e) => write!(f, "Library error: {}", e),
        }
    }
}

impl From<reqwest::Error> for PayPalError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

impl From<serde_json::Error> for PayPalError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<ValidationError> for PayPalError {
    fn from(error: ValidationError) -> Self {
        Self::Api(error)
    }
}

impl From<serde_urlencoded::ser::Error> for PayPalError {
    fn from(error: serde_urlencoded::ser::Error) -> Self {
        Self::QueryString(error)
    }
}

impl From<Error> for PayPalError {
    fn from(error: Error) -> Self {
        match error {
            Error::Reqwest(error) => Self::Http(error),
            Error::Middleware(_) => Self::LibraryError(
                "Middleware error should not be returned from PayPal API, please report this issue"
                    .to_string(),
            ),
        }
    }
}
