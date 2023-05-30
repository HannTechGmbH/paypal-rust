use serde::{Deserialize, Serialize};

/// The intent to either capture payment immediately or authorize a payment for an order after order creation.
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum OrderIntent {
    /// The merchant intends to capture payment immediately after the customer makes a payment.
    #[default]
    #[serde(rename = "CAPTURE")]
    Capture,
    /// The merchant intends to authorize a payment and place funds on hold after the customer makes a payment.
    /// Authorized payments are best captured within three days of authorization but are available to capture for up to 29 days.
    /// After the three-day honor period, the original authorized payment expires and you must re-authorize the payment.
    /// You must make a separate request to capture payments on demand. This intent is not supported when you have more than one
    /// `purchase_unit` within your order.
    #[serde(rename = "AUTHORIZE")]
    Authorize,
}

impl OrderIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Capture => "CAPTURE",
            Self::Authorize => "AUTHORIZE",
        }
    }
}

impl AsRef<str> for OrderIntent {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderIntent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
