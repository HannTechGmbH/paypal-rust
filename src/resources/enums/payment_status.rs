use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PaymentStatus {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "CAPTURED")]
    Captured,
    #[serde(rename = "DENIED")]
    Denied,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "PARTIALLY_CAPTURED")]
    PartiallyCaptured,
    #[serde(rename = "PARTIALLY_CREATED")]
    PartiallyCreated,
    #[serde(rename = "VOIDED")]
    Voided,
    #[serde(rename = "PENDING")]
    Pending,
}

impl PaymentStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "CREATED",
            Self::Captured => "CAPTURED",
            Self::Denied => "DENIED",
            Self::Expired => "EXPIRED",
            Self::PartiallyCaptured => "PARTIALLY_CAPTURED",
            Self::PartiallyCreated => "PARTIALLY_CREATED",
            Self::Voided => "VOIDED",
            Self::Pending => "PENDING",
        }
    }
}

impl AsRef<str> for PaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
