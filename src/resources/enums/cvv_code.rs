use serde::{Deserialize, Serialize};

/// The card verification value code for for Visa, Discover, Mastercard, or American Express.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CvvCode {
    /// For Visa, Mastercard, Discover, or American Express, error - unrecognized or unknown response.
    E,
    /// For Visa, Mastercard, Discover, or American Express, invalid or null.
    I,
    /// For Visa, Mastercard, Discover, or American Express, the CVV2/CSC matches.
    M,
    ///  For Visa, Mastercard, Discover, or American Express, the CVV2/CSC does not match.
    N,
    /// For Visa, Mastercard, Discover, or American Express, it was not processed.
    P,
    /// For Visa, Mastercard, Discover, or American Express, the service is not supported.
    S,
    /// For Visa, Mastercard, Discover, or American Express, unknown - the issuer is not certified.
    U,
    /// For Visa, Mastercard, Discover, or American Express, no response. For Maestro, the service is not available.
    X,
    /// For Visa, Mastercard, Discover, or American Express, error.
    #[serde(rename = "All others")]
    AllOthers,
    /// For Maestro, the CVV2 matched.
    #[serde(rename = "0")]
    Zero,
    /// For Maestro, the CVV2 did not match.
    #[serde(rename = "1")]
    One,
    /// For Maestro, the merchant has not implemented CVV2 code handling.
    #[serde(rename = "2")]
    Two,
    /// For Maestro, the merchant has indicated that CVV2 is not present on card.
    #[serde(rename = "3")]
    Three,
    /// For Maestro, the service is not available.
    #[serde(rename = "4")]
    Four,
}

impl CvvCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::E => "E",
            Self::I => "I",
            Self::M => "M",
            Self::N => "N",
            Self::P => "P",
            Self::S => "S",
            Self::U => "U",
            Self::X => "X",
            Self::AllOthers => "All others",
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
        }
    }
}

impl AsRef<str> for CvvCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CvvCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
