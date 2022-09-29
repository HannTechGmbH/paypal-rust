use crate::resources::money::Money;
use crate::resources::net_amount_breakdown::NetAmountBreakdown;
use crate::resources::platform_fee::PlatformFee;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SellerPayableBreakdown {
    /// The amount that the payee refunded to the payer.
    pub gross_amount: Money,

    /// The net amount that the payee's account is debited in the transaction currency.
    /// The net amount is calculated as gross_amount minus paypal_fee minus platform_fees.
    pub net_amount: Money,

    /// An array of breakdown values for the net amount. Returned when the currency of the refund is different from the currency of the
    /// PayPal account where the payee holds their funds.
    pub net_amount_breakdown: Option<NetAmountBreakdown>,

    /// The net amount that the payee's account is debited in the receivable currency. Returned only in cases when the receivable currency is
    /// different from transaction currency. Example 'CNY'.
    pub net_amount_in_receivable_currency: Option<Money>,

    /// The PayPal fee that was refunded to the payer in the currency of the transaction.
    /// This fee might not match the PayPal fee that the payee paid when the payment was captured.
    pub paypal_fee: Money,

    /// The PayPal fee that was refunded to the payer in the receivable currency. Returned only in cases when the receivable currency is
    /// different from transaction currency. Example 'CNY'.
    pub paypal_fee_in_receivable_currency: Option<Money>,

    /// An array of platform or partner fees, commissions, or brokerage fees for the refund.
    pub platform_fees: Vec<PlatformFee>,

    /// The total amount refunded from the original capture to date. For example, if a payer makes a $100 purchase and was refunded $20 a week
    /// ago and was refunded $30 in this refund, the gross_amount is $30 for this refund and the total_refunded_amount is $50.
    pub total_refunded_amount: Money,
}
