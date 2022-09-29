use serde::{Deserialize, Serialize};

/// Indicates whether the transaction is eligible for seller protection.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum SellerProtectionStatus {
    /// Your PayPal balance remains intact if the customer claims that they did not receive an item or the account holder claims
    /// that they did not authorize the payment.
    #[serde(rename = "ELIGIBLE")]
    Eligible,
    /// Your PayPal balance remains intact if the customer claims that they did not receive an item.
    #[serde(rename = "PARTIALLY_ELIGIBLE")]
    PartiallyEligible,
    /// This transaction is not eligible for seller protection.
    #[serde(rename = "NOT_ELIGIBLE")]
    NotEligible,
}

impl SellerProtectionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SellerProtectionStatus::Eligible => "ELIGIBLE",
            SellerProtectionStatus::PartiallyEligible => "PARTIALLY_ELIGIBLE",
            SellerProtectionStatus::NotEligible => "NOT_ELIGIBLE",
        }
    }
}

impl AsRef<str> for SellerProtectionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SellerProtectionStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
