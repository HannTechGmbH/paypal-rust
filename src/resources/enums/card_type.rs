use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CardType {
    #[serde(rename = "VISA")]
    Visa,
    #[serde(rename = "MASTERCARD")]
    Mastercard,
    #[serde(rename = "DISCOVER")]
    Discover,
    #[serde(rename = "AMEX")]
    Amex,
    #[serde(rename = "SOLO")]
    Solo,
    #[serde(rename = "JCB")]
    Jcb,
    #[serde(rename = "STAR")]
    Star,
    #[serde(rename = "DELTA")]
    Delta,
    #[serde(rename = "SWITCH")]
    Switch,
    #[serde(rename = "MAESTRO")]
    Maestro,
    #[serde(rename = "CB_NATIONALE")]
    CbNationale,
    #[serde(rename = "CONFIGOGA")]
    Configoga,
    #[serde(rename = "CONFIDIS")]
    Confidis,
    #[serde(rename = "ELECTRON")]
    Electron,
    #[serde(rename = "CETELEM")]
    Cetelem,
    #[serde(rename = "CHINA_UNION_PAY")]
    ChinaUnionPay,
}

impl CardType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Visa => "VISA",
            Self::Mastercard => "MASTERCARD",
            Self::Discover => "DISCOVER",
            Self::Amex => "AMEX",
            Self::Solo => "SOLO",
            Self::Jcb => "JCB",
            Self::Star => "STAR",
            Self::Delta => "DELTA",
            Self::Switch => "SWITCH",
            Self::Maestro => "MAESTRO",
            Self::CbNationale => "CB_NATIONALE",
            Self::Configoga => "CONFIGOGA",
            Self::Confidis => "CONFIDIS",
            Self::Electron => "ELECTRON",
            Self::Cetelem => "CETELEM",
            Self::ChinaUnionPay => "CHINA_UNION_PAY",
        }
    }
}

impl AsRef<str> for CardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
