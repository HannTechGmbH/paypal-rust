use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum DisbursementMode {
    #[serde(rename = "INSTANT")]
    Instant,
    #[serde(rename = "DELAYED")]
    Delayed,
}

impl DisbursementMode {
    pub fn as_str(self) -> &'static str {
        match self {
            DisbursementMode::Instant => "INSTANT",
            DisbursementMode::Delayed => "DELAYED",
        }
    }
}

impl Default for DisbursementMode {
    fn default() -> Self {
        DisbursementMode::Instant
    }
}

impl AsRef<str> for DisbursementMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisbursementMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
