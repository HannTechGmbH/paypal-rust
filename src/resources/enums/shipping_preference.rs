use serde::{Deserialize, Serialize};

/// The shipping preference:
///  * Displays the shipping address to the customer.
///  * Enables the customer to choose an address on the PayPal site.
///  * Restricts the customer from changing the address during the payment-approval process.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ShippingPreference {
    /// Use the customer-provided shipping address on the PayPal site.
    #[serde(rename = "GET_FROM_FILE")]
    GetFromFile,
    /// Redact the shipping address from the PayPal site. Recommended for digital goods.
    #[serde(rename = "NO_SHIPPING")]
    NoShipping,
    /// Use the merchant-provided address. The customer cannot change this address on the PayPal site.
    #[serde(rename = "SET_PROVIDED_ADDRESS")]
    SetProvidedAddress,
}

impl ShippingPreference {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingPreference::GetFromFile => "GET_FROM_FILE",
            ShippingPreference::NoShipping => "NO_SHIPPING",
            ShippingPreference::SetProvidedAddress => "SET_PROVIDED_ADDRESS",
        }
    }
}

impl AsRef<str> for ShippingPreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingPreference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
