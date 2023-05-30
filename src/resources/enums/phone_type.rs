use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PhoneType {
    #[serde(rename = "FAX")]
    Fax,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "MOBILE")]
    Mobile,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "PAGER")]
    Pager,
}

impl PhoneType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fax => "FAX",
            Self::Home => "HOME",
            Self::Mobile => "MOBILE",
            Self::Other => "OTHER",
            Self::Pager => "PAGER",
        }
    }
}

impl AsRef<str> for PhoneType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PhoneType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
