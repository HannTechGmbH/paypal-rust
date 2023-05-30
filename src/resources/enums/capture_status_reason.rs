use serde::{Deserialize, Serialize};

/// The reason why the captured payment status is PENDING or DENIED.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CaptureStatusReason {
    /// The payer initiated a dispute for this captured payment with PayPal.
    #[serde(rename = "BUYER_COMPLAINT")]
    BuyerComplaint,
    /// The captured funds were reversed in response to the payer disputing this captured payment with the issuer of the
    /// financial instrument used to pay for this captured payment.
    #[serde(rename = "CHARGEBACK")]
    Chargeback,
    /// The payer paid by an eCheck that has not yet cleared.
    #[serde(rename = "ECHECK")]
    Echeck,
    /// Visit your online account. In your **Account Overview**, accept and deny this payment.
    #[serde(rename = "INTERNATIONAL_WITHDRAWAL")]
    InternationalWithdrawal,
    /// No additional specific reason can be provided. For more information about this captured payment, visit your account online
    /// or contact PayPal.
    #[serde(rename = "OTHER")]
    Other,
    /// The captured payment is pending manual review.
    #[serde(rename = "PENDING_REVIEW")]
    PendingReview,
    /// The payee has not yet set up appropriate receiving preferences for their account.
    /// For more information about how to accept or deny this payment, visit your account online. This reason is typically offered in
    /// scenarios such as when the currency of the captured payment is different from the primary holding currency of the payee.
    #[serde(rename = "RECEIVING_PREFERENCE_MANDATES_MANUAL_ACTION")]
    ReceivingPreferenceMandatesManualAction,
    /// The captured funds were refunded.
    #[serde(rename = "REFUNDED")]
    Refunded,
    /// The payer must send the funds for this captured payment. This code generally appears for
    /// manual EFTs.
    #[serde(rename = "TRANSACTION_APPROVED_AWAITING_FUNDING")]
    TransactionApprovedAwaitingFunding,
    /// The payee does not have a PayPal account.
    #[serde(rename = "UNILATERAL")]
    Unilateral,
    /// The payee's PayPal account is not verified.
    #[serde(rename = "VERIFICATION_REQUIRED")]
    VerificationRequired,
}

impl CaptureStatusReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuyerComplaint => "BUYER_COMPLAINT",
            Self::Chargeback => "CHARGEBACK",
            Self::Echeck => "ECHECK",
            Self::InternationalWithdrawal => "INTERNATIONAL_WITHDRAWAL",
            Self::Other => "OTHER",
            Self::PendingReview => "PENDING_REVIEW",
            Self::ReceivingPreferenceMandatesManualAction => {
                "RECEIVING_PREFERENCE_MANDATES_MANUAL_ACTION"
            }
            Self::Refunded => "REFUNDED",
            Self::TransactionApprovedAwaitingFunding => "TRANSACTION_APPROVED_AWAITING_FUNDING",
            Self::Unilateral => "UNILATERAL",
            Self::VerificationRequired => "VERIFICATION_REQUIRED",
        }
    }
}

impl AsRef<str> for CaptureStatusReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CaptureStatusReason {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
