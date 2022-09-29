use crate::resources::enums::dispute_category::DisputeCategory;
use crate::resources::enums::seller_protection_status::SellerProtectionStatus;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SellerProtection {
    /// Indicates whether the transaction is eligible for seller protection.
    pub status: Option<SellerProtectionStatus>,

    /// An array of conditions that are covered for the transaction.
    pub dispute_categories: Option<Vec<DisputeCategory>>,
}
