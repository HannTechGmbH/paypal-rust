use serde::{Deserialize, Serialize};

/// Configures a Continue or Pay Now checkout flow.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum UserAction {
    /// After you redirect the customer to the PayPal payment page, a Continue button appears. Use this option when the final
    /// amount is not known when the checkout flow is initiated and you want to redirect the customer to the merchant page without processing
    #[serde(rename = "CONTINUE")]
    Continue,
    /// After you redirect the customer to the PayPal payment page, a Pay Now button appears. Use this option when the final
    /// amount is known when the checkout is initiated and you want to process the payment immediately when the customer clicks Pay Now.
    #[serde(rename = "PAY_NOW")]
    PayNow,
}

impl UserAction {
    pub fn as_str(self) -> &'static str {
        match self {
            UserAction::Continue => "CONTINUE",
            UserAction::PayNow => "PAY_NOW",
        }
    }
}

impl AsRef<str> for UserAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UserAction {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
