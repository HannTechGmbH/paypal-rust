use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Network {
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

impl Network {
    pub fn as_str(self) -> &'static str {
        match self {
            Network::Visa => "VISA",
            Network::Mastercard => "MASTERCARD",
            Network::Discover => "DISCOVER",
            Network::Amex => "AMEX",
            Network::Solo => "SOLO",
            Network::Jcb => "JCB",
            Network::Star => "STAR",
            Network::Delta => "DELTA",
            Network::Switch => "SWITCH",
            Network::Maestro => "MAESTRO",
            Network::CbNationale => "CB_NATIONALE",
            Network::Configoga => "CONFIGOGA",
            Network::Confidis => "CONFIDIS",
            Network::Electron => "ELECTRON",
            Network::Cetelem => "CETELEM",
            Network::ChinaUnionPay => "CHINA_UNION_PAY",
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Visa
    }
}

impl AsRef<str> for Network {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
