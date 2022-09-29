use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CurrencyCode {
    #[serde(rename = "AUD")]
    AustralianDollar,
    #[serde(rename = "BRL")]
    BrazilianReal,
    #[serde(rename = "CAD")]
    CanadianDollar,
    #[serde(rename = "CNY")]
    ChineseRenmenbi,
    #[serde(rename = "CZK")]
    CzechKoruna,
    #[serde(rename = "DKK")]
    DanishKrone,
    #[serde(rename = "EUR")]
    Euro,
    #[serde(rename = "HKD")]
    HongKongDollar,
    #[serde(rename = "HUF")]
    HungarianForint,
    #[serde(rename = "ILS")]
    IsraeliNewShekel,
    #[serde(rename = "JPY")]
    JapaneseYen,
    #[serde(rename = "MYR")]
    MalaysianRinggit,
    #[serde(rename = "MXN")]
    MexicanPeso,
    #[serde(rename = "TWD")]
    NewTaiwanDollar,
    #[serde(rename = "NZD")]
    NewZealandDollar,
    #[serde(rename = "NOK")]
    Norwegiankrone,
    #[serde(rename = "PHP")]
    PhilippinePeso,
    #[serde(rename = "PLN")]
    PolishZloty,
    #[serde(rename = "GBP")]
    PoundSterling,
    #[serde(rename = "RUB")]
    RussianRuble,
    #[serde(rename = "SGD")]
    SingaporeDollar,
    #[serde(rename = "SEK")]
    SwedishKrona,
    #[serde(rename = "CHF")]
    SwissFranc,
    #[serde(rename = "THB")]
    ThaiBaht,
    #[serde(rename = "USD")]
    UnitedStatesDollar,
}

impl CurrencyCode {
    pub fn as_str(self) -> &'static str {
        match self {
            CurrencyCode::AustralianDollar => "AUD",
            CurrencyCode::BrazilianReal => "BRL",
            CurrencyCode::CanadianDollar => "CAD",
            CurrencyCode::ChineseRenmenbi => "CNY",
            CurrencyCode::CzechKoruna => "CZK",
            CurrencyCode::DanishKrone => "DKK",
            CurrencyCode::Euro => "EUR",
            CurrencyCode::HongKongDollar => "HKD",
            CurrencyCode::HungarianForint => "HUF",
            CurrencyCode::IsraeliNewShekel => "ILS",
            CurrencyCode::JapaneseYen => "JPY",
            CurrencyCode::MalaysianRinggit => "MYR",
            CurrencyCode::MexicanPeso => "MXN",
            CurrencyCode::NewTaiwanDollar => "TWD",
            CurrencyCode::NewZealandDollar => "NZD",
            CurrencyCode::Norwegiankrone => "NOK",
            CurrencyCode::PhilippinePeso => "PHP",
            CurrencyCode::PolishZloty => "PLN",
            CurrencyCode::PoundSterling => "GBP",
            CurrencyCode::RussianRuble => "RUB",
            CurrencyCode::SingaporeDollar => "SGD",
            CurrencyCode::SwedishKrona => "SEK",
            CurrencyCode::SwissFranc => "CHF",
            CurrencyCode::ThaiBaht => "THB",
            CurrencyCode::UnitedStatesDollar => "USD",
        }
    }
}

impl Default for CurrencyCode {
    fn default() -> Self {
        CurrencyCode::Euro
    }
}

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}

impl FromStr for CurrencyCode {
    type Err = ParseCurrencyCodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AUD" => Ok(CurrencyCode::AustralianDollar),
            "BRL" => Ok(CurrencyCode::BrazilianReal),
            "CAD" => Ok(CurrencyCode::CanadianDollar),
            "CNY" => Ok(CurrencyCode::ChineseRenmenbi),
            "CZK" => Ok(CurrencyCode::CzechKoruna),
            "DKK" => Ok(CurrencyCode::DanishKrone),
            "EUR" => Ok(CurrencyCode::Euro),
            "HKD" => Ok(CurrencyCode::HongKongDollar),
            "HUF" => Ok(CurrencyCode::HungarianForint),
            "ILS" => Ok(CurrencyCode::IsraeliNewShekel),
            "JPY" => Ok(CurrencyCode::JapaneseYen),
            "MYR" => Ok(CurrencyCode::MalaysianRinggit),
            "MXN" => Ok(CurrencyCode::MexicanPeso),
            "TWD" => Ok(CurrencyCode::NewTaiwanDollar),
            "NZD" => Ok(CurrencyCode::NewZealandDollar),
            "NOK" => Ok(CurrencyCode::Norwegiankrone),
            "PHP" => Ok(CurrencyCode::PhilippinePeso),
            "PLN" => Ok(CurrencyCode::PolishZloty),
            "GBP" => Ok(CurrencyCode::PoundSterling),
            "RUB" => Ok(CurrencyCode::RussianRuble),
            "SGD" => Ok(CurrencyCode::SingaporeDollar),
            "SEK" => Ok(CurrencyCode::SwedishKrona),
            "CHF" => Ok(CurrencyCode::SwissFranc),
            "THB" => Ok(CurrencyCode::ThaiBaht),
            "USD" => Ok(CurrencyCode::UnitedStatesDollar),
            _ => Err(ParseCurrencyCodeError(())),
        }
    }
}

#[derive(Debug)]
pub struct ParseCurrencyCodeError(/* private */ ());

impl std::fmt::Display for ParseCurrencyCodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "invalid currency code".fmt(formatter)
    }
}

impl std::error::Error for ParseCurrencyCodeError {
    fn description(&self) -> &str {
        "invalid currency code"
    }
}
