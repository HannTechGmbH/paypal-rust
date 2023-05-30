use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum DisbursementMode {
    #[default]
    #[serde(rename = "INSTANT")]
    Instant,
    #[serde(rename = "DELAYED")]
    Delayed,
}

impl DisbursementMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "INSTANT",
            Self::Delayed => "DELAYED",
        }
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
