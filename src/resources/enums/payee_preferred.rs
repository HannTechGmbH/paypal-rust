use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PayeePreferred {
    #[serde(rename = "UNRESTRICTED")]
    Unrestricted,
    #[serde(rename = "IMMEDIATE_PAYMENT_REQUIRED")]
    ImmediatePaymentRequired,
}

impl PayeePreferred {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unrestricted => "UNRESTRICTED",
            Self::ImmediatePaymentRequired => "IMMEDIATE_PAYMENT_REQUIRED",
        }
    }
}

impl AsRef<str> for PayeePreferred {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayeePreferred {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
