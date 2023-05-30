use serde::{Deserialize, Serialize};

/// Indicates if this is a first or subsequent payment using a stored payment source
/// (also referred to as stored credential or card on file).
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Usage {
    /// Indicates the Initial/First payment with a payment_source that is intended to be stored upon
    //  successful processing of the payment.
    #[serde(rename = "FIRST")]
    First,
    ///  Indicates a payment using a stored payment_source which has been successfully used previously for a payment.
    #[serde(rename = "SUBSEQUENT")]
    Subsequent,
    /// Indicates that PayPal will derive the value of `FIRST` or `SUBSEQUENT` based on data available to PayPal.
    #[serde(rename = "Derived")]
    Derived,
}

impl Usage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::First => "FIRST",
            Self::Subsequent => "SUBSEQUENT",
            Self::Derived => "Derived",
        }
    }
}

impl AsRef<str> for Usage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Usage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
