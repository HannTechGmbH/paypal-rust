use crate::resources::enums::disembursement_mode::DisbursementMode;
use crate::resources::platform_fee::PlatformFee;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentInstruction {
    /// An array of various fees, commissions, tips, or donations. This field is only applicable to merchants that been enabled for PayPal
    /// Commerce Platform for Marketplaces and Platforms capability.
    pub platform_fees: Vec<PlatformFee>,

    /// The funds that are held on behalf of the merchant.
    pub disbursement_mode: DisbursementMode,

    pub payee_pricing_tier_id: String,
}
