use serde::{Deserialize, Serialize};

/// The address verification code for Visa, Discover, Mastercard, or American Express transactions.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AvsCode {
    /// For Visa, Mastercard, or Discover transactions, the address matches but the zip code does not match. For American Express transactions, the card holder address is correct.
    A,
    /// For Visa, Mastercard, or Discover transactions, the address matches. International A.
    B,
    /// For Visa, Mastercard, or Discover transactions, no values match. International N.
    C,
    /// For Visa, Mastercard, or Discover transactions, the address and postal code match. International X.
    D,
    /// For Visa, Mastercard, or Discover transactions, not allowed for Internet or phone transactions. For American Express card holder, the name is incorrect but the address and postal code match.
    E,
    /// For Visa, Mastercard, or Discover transactions, the address and postal code match. UK-specific X. For American Express card
    /// holder, the name is incorrect but the address matches.
    F,
    /// For Visa, Mastercard, or Discover transactions, global is unavailable. Nothing matches.
    G,
    /// For Visa, Mastercard, or Discover transactions, international is unavailable. Not applicable.
    I,
    /// For Visa, Mastercard, or Discover transactions, the address and postal code match. For American Express card holder, the name, address, and postal code match.
    M,
    /// For Visa, Mastercard, or Discover transactions, nothing matches. For American Express card holder, the address and postal code are both incorrect.
    N,
    /// For Visa, Mastercard, or Discover transactions, postal international Z. Postal code only.
    P,
    /// For Visa, Mastercard, or Discover transactions, re-try the request. For American Express, the system is unavailable.
    R,
    /// For Visa, Mastercard, Discover, or American Express, the service is not supported.
    S,
    /// For Visa, Mastercard, or Discover transactions, the service is unavailable. For American Express, information is not available.
    /// For Maestro, the address is not checked or the acquirer had no response. The service is not available.
    U,
    /// For Visa, Mastercard, or Discover transactions, whole ZIP code. For American Express, the card holder name, address, and postal
    /// code are all incorrect.
    W,
    /// For Visa, Mastercard, or Discover transactions, exact match of the address and the nine-digit ZIP code. For American Express,
    /// the card holder name, address, and postal code are all incorrect.
    X,
    ///  For Visa, Mastercard, or Discover transactions, the address and five-digit ZIP code match. For American Express, the card holder
    /// address and postal code are both correct.
    Y,
    /// For Visa, Mastercard, or Discover transactions, the five-digit ZIP code matches but no address. For American Express, only the
    /// card holder postal code is correct.
    Z,
    /// For Maestro, no AVS response was obtained.
    Null,
    /// For Maestro, all address information matches.
    #[serde(rename = "0")]
    Zero,
    /// - 1. For Maestro, none of the address information matches.
    #[serde(rename = "1")]
    One,
    /// - 2. For Maestro, part of the address information matches.
    #[serde(rename = "2")]
    Two,
    /// - 3. For Maestro, the merchant did not provide AVS information. It was not processed.
    #[serde(rename = "3")]
    Three,
    /// - 4. For Maestro, the address was not checked or the acquirer had no response. The service is not available.
    #[serde(rename = "4")]
    Four,
}

impl AvsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AvsCode::A => "A",
            AvsCode::B => "B",
            AvsCode::C => "C",
            AvsCode::D => "D",
            AvsCode::E => "E",
            AvsCode::F => "F",
            AvsCode::G => "G",
            AvsCode::I => "I",
            AvsCode::M => "M",
            AvsCode::N => "N",
            AvsCode::P => "P",
            AvsCode::R => "R",
            AvsCode::S => "S",
            AvsCode::U => "U",
            AvsCode::W => "W",
            AvsCode::X => "X",
            AvsCode::Y => "Y",
            AvsCode::Z => "Z",
            AvsCode::Null => "Null",
            AvsCode::Zero => "0",
            AvsCode::One => "1",
            AvsCode::Two => "2",
            AvsCode::Three => "3",
            AvsCode::Four => "4",
        }
    }
}

impl AsRef<str> for AvsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AvsCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
