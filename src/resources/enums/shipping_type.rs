use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ShippingType {
    #[serde(rename = "SHIPPING")]
    Shipping,
    #[serde(rename = "PICKUP_IN_PERSON")]
    PickupInPerson,
}

impl ShippingType {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingType::Shipping => "SHIPPING",
            ShippingType::PickupInPerson => "PICKUP_IN_PERSON",
        }
    }
}

impl Default for ShippingType {
    fn default() -> Self {
        ShippingType::Shipping
    }
}

impl AsRef<str> for ShippingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
