use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum RefundStatus {
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "COMPLETED")]
    Completed,
}

impl RefundStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            RefundStatus::Cancelled => "CANCELLED",
            RefundStatus::Pending => "PENDING",
            RefundStatus::Completed => "COMPLETED",
        }
    }
}

impl AsRef<str> for RefundStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefundStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
