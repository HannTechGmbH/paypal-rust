use crate::resources::exchange_rate::ExchangeRate;
use crate::resources::money::Money;
use crate::resources::platform_fee::PlatformFee;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SellerReceivableBreakdown {
    /// The amount for this captured payment in the currency of the transaction.
    pub gross_amount: Money,

    /// The applicable fee for this captured payment in the currency of the transaction.
    pub paypal_fee: Option<Money>,

    /// The applicable fee for this captured payment in the receivable currency. Returned only in cases the fee is charged in the receivable
    /// currency. Example 'CNY'.
    pub paypal_fee_in_receivable_currency: Option<Money>,

    /// The net amount that the payee receives for this captured payment in their PayPal account. The net amount is computed as gross_amount
    /// minus the paypal_fee minus the platform_fees.
    pub net_amount: Option<Money>,

    /// The net amount that is credited to the payee's PayPal account. Returned only when the currency of the captured payment is different
    /// from the currency of the PayPal account where the payee wants to credit the funds. The amount is computed as net_amount times
    /// exchange_rate.
    pub receivable_amount: Option<Money>,

    /// The exchange rate that determines the amount that is credited to the payee's PayPal account. Returned when the currency of the
    /// captured payment is different from the currency of the PayPal account where the payee wants to credit the funds.
    pub exchange_rate: Option<ExchangeRate>,

    /// An array of platform or partner fees, commissions, or brokerage fees that associated with the captured payment.
    pub platform_fees: Option<Vec<PlatformFee>>,
}
