use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum ShippingType {
    #[default]
    #[serde(rename = "SHIPPING")]
    Shipping,
    #[serde(rename = "PICKUP_IN_PERSON")]
    PickupInPerson,
}

impl ShippingType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shipping => "SHIPPING",
            Self::PickupInPerson => "PICKUP_IN_PERSON",
        }
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
