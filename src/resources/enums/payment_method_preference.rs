use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
/// The merchant-preferred payment methods.
pub enum PaymentMethodPreference {
    /// Accepts any type of payment from the customer.
    #[serde(rename = "UNRESTRICTED")]
    Unrestricted,
    /// Accepts only immediate payment from the customer. For example, credit card, PayPal balance,
    /// or instant ACH. Ensures that at the time of capture, the payment does not have the `pending`
    /// status.
    #[serde(rename = "IMMEDIATE_PAYMENT_REQUIRED")]
    ImmediatePaymentRequired,
}
