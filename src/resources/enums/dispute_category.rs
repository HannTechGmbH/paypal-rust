use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum DisputeCategory {
    #[serde(rename = "ITEM_NOT_RECEIVED")]
    ItemNotReceived,
    #[serde(rename = "UNAUTHORIZED_TRANSACTION")]
    UnauthorizedTransaction,
}

impl DisputeCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputeCategory::ItemNotReceived => "ITEM_NOT_RECEIVED",
            DisputeCategory::UnauthorizedTransaction => "UNAUTHORIZED_TRANSACTION",
        }
    }
}

impl AsRef<str> for DisputeCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeCategory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
