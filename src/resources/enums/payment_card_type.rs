use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PaymentCardType {
    #[serde(rename = "CREDIT")]
    Credit,
    #[serde(rename = "DEBIT")]
    Debit,
    #[serde(rename = "PREPAID")]
    Prepaid,
    #[serde(rename = "STORE")]
    Store,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl PaymentCardType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Credit => "CREDIT",
            Self::Debit => "DEBIT",
            Self::Prepaid => "PREPAID",
            Self::Store => "STORE",
            Self::Unknown => "UNKNOWN",
        }
    }
}

impl AsRef<str> for PaymentCardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentCardType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
