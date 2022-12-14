use crate::resources::capture_status_details::CaptureStatusDetails;
use crate::resources::enums::disembursement_mode::DisbursementMode;
use crate::resources::link_description::LinkDescription;
use crate::resources::money::Money;
use crate::resources::processor_response::ProcessorResponse;
use crate::resources::seller_protection::SellerProtection;
use crate::resources::seller_recievable_breakdown::SellerReceivableBreakdown;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Capture {
    /// The status of the captured payment.
    pub status: String,

    /// The details of the captured payment status.
    pub status_details: Option<CaptureStatusDetails>,

    /// The amount for this captured payment.
    pub amount: Money,

    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions with PayPal transactions.
    /// Appears in transaction and settlement reports.
    pub custom_id: String,

    /// The funds that are held on behalf of the merchant.
    pub disbursement_mode: Option<DisbursementMode>,

    /// Indicates whether you can make additional captures against the authorized payment. Set to true if you do not intend to capture
    /// additional payments against the authorization. Set to false if you intend to capture additional payments against the authorization.
    pub final_capture: bool,

    /// The PayPal-generated ID for the captured payment.
    pub id: String,

    /// The API caller-provided external invoice number for this order. Appears in both the payer's transaction history and the
    /// emails that the payer receives.
    pub invoice_id: Option<String>,

    /// An array of related HATEOAS links.
    pub links: Vec<LinkDescription>,

    /// An object that provides additional processor information for a direct credit card transaction.
    pub processor_response: Option<ProcessorResponse>,

    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: SellerProtection,

    /// The detailed breakdown of the capture activity. This is not available for transactions that are in pending state.
    pub seller_receivable_breakdown: SellerReceivableBreakdown,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: String,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: String,
}
