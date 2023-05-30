use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AuthorizationStatusReason {
    #[serde(rename = "PENDING_REVIEW")]
    PendingReview,
}

impl AuthorizationStatusReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "PENDING_REVIEW",
        }
    }
}

impl AsRef<str> for AuthorizationStatusReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AuthorizationStatusReason {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
