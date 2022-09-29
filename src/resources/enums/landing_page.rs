use serde::{Deserialize, Serialize};

/// The type of landing page to show on the PayPal site for customer checkout.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum LandingPage {
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page to log in to PayPal and approve the payment.
    #[serde(rename = "LOGIN")]
    Login,
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page to enter credit or debit card and other
    /// relevant billing information required to complete the purchase.
    #[serde(rename = "BILLING")]
    Billing,
    /// When the customer clicks PayPal Checkout, the customer is redirected to either a page to log in to PayPal and
    /// approve the payment or to a page to enter credit or debit card and other relevant billing information required to complete the
    /// purchase, depending on their previous interaction with PayPal.
    #[serde(rename = "NO_PREFERENCE")]
    NoPreference,
}

impl LandingPage {
    pub fn as_str(self) -> &'static str {
        match self {
            LandingPage::Login => "LOGIN",
            LandingPage::Billing => "BILLING",
            LandingPage::NoPreference => "NO_PREFERENCE",
        }
    }
}

impl AsRef<str> for LandingPage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LandingPage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
