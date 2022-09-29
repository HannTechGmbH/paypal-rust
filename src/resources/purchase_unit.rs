use crate::resources::amount_with_breakdown::AmountWithBreakdown;
use crate::resources::item::Item;
use crate::resources::payee::Payee;
use crate::resources::payment_collection::PaymentCollection;
use crate::resources::payment_instruction::PaymentInstruction;
use crate::resources::shipping_detail::ShippingDetail;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PurchaseUnit {
    /// The API caller-provided external ID for the purchase unit. Required for multiple purchase units when you must update the order
    /// through PATCH. If you omit this value and the order contains only one purchase unit, PayPal sets this value to default.
    pub reference_id: Option<String>,

    /// The total order amount with an optional breakdown that provides details, such as the total item amount, total tax amount, shipping,
    /// handling, insurance, and discounts, if any. If you specify amount.breakdown, the amount equals item_total plus tax_total plus
    /// shipping plus handling plus insurance minus shipping_discount minus discount. The amount must be a positive number.
    /// For listed of supported currencies and decimal precision, see the PayPal REST APIs Currency Codes.
    pub amount: Option<AmountWithBreakdown>,

    /// The merchant who receives payment for this transaction.
    pub payee: Option<Payee>,

    /// Any additional payment instructions to be consider during payment processing. This processing instruction is applicable for
    /// Capturing an order or Authorizing an Order.
    pub payment_instruction: Option<PaymentInstruction>,

    /// The purchase description.
    pub description: Option<String>,

    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions with PayPal transactions.
    /// Appears in transaction and settlement reports.
    pub custom_id: Option<String>,

    /// The API caller-provided external invoice ID for this order.
    pub invoice_id: Option<String>,

    /// The PayPal-generated ID for the purchase unit. This ID appears in both the payer's transaction history and the emails that the payer
    /// receives. In addition, this ID is available in transaction and settlement reports that merchants and API callers can use to reconcile
    /// transactions. This ID is only available when an order is saved by calling v2/checkout/orders/id/save.
    pub id: Option<String>,

    /// The payment descriptor on account transactions on the customer's credit card statement, that PayPal sends to processors.
    /// The maximum length of the soft descriptor information that you can pass in the API field is 22 characters, in the following
    /// format:22 - len(PAYPAL * (8)) - len(Descriptor in Payment Receiving Preferences of Merchant account + 1)The PAYPAL prefix uses 8
    /// characters.
    ///
    /// The soft descriptor supports the following ASCII characters:
    /// - Alphanumeric characters
    /// - Dashes
    /// - Asterisks
    /// - Periods (.)
    /// - Spaces
    ///
    /// For Wallet payments marketplace integrations:
    /// The merchant descriptor in the Payment Receiving Preferences must be the marketplace name.
    /// You can't use the remaining space to show the customer service number.
    /// The remaining spaces can be a combination of seller name and country.
    ///
    /// For unbranded payments (Direct Card) marketplace integrations, use a combination of the seller name and phone number.
    pub soft_descriptor: Option<String>,

    /// An array of items that the customer purchases from the merchant.
    pub items: Option<Vec<Item>>,

    /// The shipping address and method.
    pub shipping: Option<ShippingDetail>,

    /// The comprehensive history of payments for the purchase unit.
    pub payments: Option<PaymentCollection>,
}
