use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum StandardEntryClassCode {
    #[serde(rename = "TEL")]
    Tel,
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "CCD")]
    Ccd,
    #[serde(rename = "PPD")]
    Ppd,
}

impl StandardEntryClassCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tel => "TEL",
            Self::Web => "WEB",
            Self::Ccd => "CCD",
            Self::Ppd => "PPD",
        }
    }
}

impl AsRef<str> for StandardEntryClassCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StandardEntryClassCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
