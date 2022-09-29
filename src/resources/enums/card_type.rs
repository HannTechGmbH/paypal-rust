use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

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
    pub fn as_str(&self) -> &'static str {
        match self {
            CardType::Visa => "VISA",
            CardType::Mastercard => "MASTERCARD",
            CardType::Discover => "DISCOVER",
            CardType::Amex => "AMEX",
            CardType::Solo => "SOLO",
            CardType::Jcb => "JCB",
            CardType::Star => "STAR",
            CardType::Delta => "DELTA",
            CardType::Switch => "SWITCH",
            CardType::Maestro => "MAESTRO",
            CardType::CbNationale => "CB_NATIONALE",
            CardType::Configoga => "CONFIGOGA",
            CardType::Confidis => "CONFIDIS",
            CardType::Electron => "ELECTRON",
            CardType::Cetelem => "CETELEM",
            CardType::ChinaUnionPay => "CHINA_UNION_PAY",
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
