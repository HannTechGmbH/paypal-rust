use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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
    #[default]
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
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AustralianDollar => "AUD",
            Self::BrazilianReal => "BRL",
            Self::CanadianDollar => "CAD",
            Self::ChineseRenmenbi => "CNY",
            Self::CzechKoruna => "CZK",
            Self::DanishKrone => "DKK",
            Self::Euro => "EUR",
            Self::HongKongDollar => "HKD",
            Self::HungarianForint => "HUF",
            Self::IsraeliNewShekel => "ILS",
            Self::JapaneseYen => "JPY",
            Self::MalaysianRinggit => "MYR",
            Self::MexicanPeso => "MXN",
            Self::NewTaiwanDollar => "TWD",
            Self::NewZealandDollar => "NZD",
            Self::Norwegiankrone => "NOK",
            Self::PhilippinePeso => "PHP",
            Self::PolishZloty => "PLN",
            Self::PoundSterling => "GBP",
            Self::RussianRuble => "RUB",
            Self::SingaporeDollar => "SGD",
            Self::SwedishKrona => "SEK",
            Self::SwissFranc => "CHF",
            Self::ThaiBaht => "THB",
            Self::UnitedStatesDollar => "USD",
        }
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
            "AUD" => Ok(Self::AustralianDollar),
            "BRL" => Ok(Self::BrazilianReal),
            "CAD" => Ok(Self::CanadianDollar),
            "CNY" => Ok(Self::ChineseRenmenbi),
            "CZK" => Ok(Self::CzechKoruna),
            "DKK" => Ok(Self::DanishKrone),
            "EUR" => Ok(Self::Euro),
            "HKD" => Ok(Self::HongKongDollar),
            "HUF" => Ok(Self::HungarianForint),
            "ILS" => Ok(Self::IsraeliNewShekel),
            "JPY" => Ok(Self::JapaneseYen),
            "MYR" => Ok(Self::MalaysianRinggit),
            "MXN" => Ok(Self::MexicanPeso),
            "TWD" => Ok(Self::NewTaiwanDollar),
            "NZD" => Ok(Self::NewZealandDollar),
            "NOK" => Ok(Self::Norwegiankrone),
            "PHP" => Ok(Self::PhilippinePeso),
            "PLN" => Ok(Self::PolishZloty),
            "GBP" => Ok(Self::PoundSterling),
            "RUB" => Ok(Self::RussianRuble),
            "SGD" => Ok(Self::SingaporeDollar),
            "SEK" => Ok(Self::SwedishKrona),
            "CHF" => Ok(Self::SwissFranc),
            "THB" => Ok(Self::ThaiBaht),
            "USD" => Ok(Self::UnitedStatesDollar),
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
