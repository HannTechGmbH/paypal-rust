use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CaptureStatus {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "PARTIALLY_REFUNDED")]
    PartiallyRefunded,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "FAILED")]
    Failed,
}

impl CaptureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CaptureStatus::Completed => "COMPLETED",
            CaptureStatus::Declined => "DECLINED",
            CaptureStatus::PartiallyRefunded => "PARTIALLY_REFUNDED",
            CaptureStatus::Pending => "PENDING",
            CaptureStatus::Refunded => "REFUNDED",
            CaptureStatus::Failed => "FAILED",
        }
    }
}

impl AsRef<str> for CaptureStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CaptureStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
