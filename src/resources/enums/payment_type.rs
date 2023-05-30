use serde::{Deserialize, Serialize};

/// Indicates the type of the stored payment_source payment.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PaymentType {
    /// One Time payment such as online purchase or donation. (e.g. Checkout with one-click).
    #[serde(rename = "ONE_TIME")]
    OneTime,
    /// Payment which is part of a series of payments with fixed or variable amounts, following a fixed time interval.
    /// (e.g. Subscription payments).
    #[serde(rename = "RECURRING")]
    Recurring,
    /// Payment which is part of a series of payments that occur on a non-fixed schedule and/or have variable amounts.
    #[serde(rename = "UNSCHEDULED")]
    Unscheduled,
}

impl Default for PaymentType {
    fn default() -> Self {
        Self::OneTime
    }
}

impl PaymentType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneTime => "ONE_TIME",
            Self::Recurring => "RECURRING",
            Self::Unscheduled => "UNSCHEDULED",
        }
    }
}

impl AsRef<str> for PaymentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
