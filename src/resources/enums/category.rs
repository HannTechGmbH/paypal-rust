use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Category {
    #[serde(rename = "DIGITAL_GOODS")]
    DigitalGoods,
    #[serde(rename = "PHYSICAL_GOODS")]
    PhysicalGoods,
    #[serde(rename = "DONATION")]
    Donation,
}

impl Category {
    pub fn as_str(self) -> &'static str {
        match self {
            Category::DigitalGoods => "DIGITAL_GOODS",
            Category::PhysicalGoods => "PHYSICAL_GOODS",
            Category::Donation => "DONATION",
        }
    }
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
