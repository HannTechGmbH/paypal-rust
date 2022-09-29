use crate::resources::amount_with_breakdown::AmountWithBreakdown;
use crate::resources::item::Item;
use crate::resources::payee::Payee;
use crate::resources::payment_instruction::PaymentInstruction;
use crate::resources::shipping_detail::ShippingDetail;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PurchaseUnitRequest {
    /// The API caller-provided external ID for the purchase unit.
    /// Required for multiple purchase units when you must update the order through PATCH.
    /// If you omit this value and the order contains only one purchase unit, PayPal sets this value to default.
    pub reference_ids: Option<String>,

    /// The total order amount with an optional breakdown that provides details,
    /// such as the total item amount, total tax amount, shipping, handling,
    /// insurance, and discounts, if any. If you specify amount.breakdown, the
    /// amount equals `item_total plus tax_total plus shipping plus handling plus insurance minus shipping_discount minus discount.`
    /// The amount must be a positive number. For listed of supported currencies
    /// and decimal precision, see the PayPal REST APIs Currency Codes.
    pub amount: AmountWithBreakdown,

    /// The merchant who receives payment for this transaction.
    pub payee: Option<Payee>,

    /// Any additional payment instructions to be consider during payment processing.
    /// This processing instruction is applicable for Capturing an order or Authorizing an Order.
    pub payment_instruction: Option<PaymentInstruction>,

    /// The purchase description.
    pub description: Option<String>,

    /// The API caller-provided external ID. Used to reconcile client transactions with PayPal transactions.
    /// Appears in transaction and settlement reports but is not visible to the payer.
    pub custom_id: Option<String>,

    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,

    /// The soft descriptor is the dynamic text used to construct the statement descriptor that appears on a payer's card statement.
    ///
    /// If an Order is paid using the "PayPal Wallet", the statement descriptor will appear in following format
    /// on the payer's card statement: `PAYPAL_prefix+(space)+merchant_descriptor+(space)+ soft_descriptor`
    ///
    /// **Note:** The merchant descriptor is the descriptor of the merchant’s payment receiving preferences which can be seen by
    /// logging into the merchant account https://www.sandbox.paypal.com/businessprofile/settings/info/edit
    ///
    /// The PAYPAL prefix uses 8 characters. Only the first 22 characters will be displayed in the statement.
    /// For example, if:
    /// The PayPal prefix toggle is PAYPAL .
    /// The merchant descriptor in the profile is Janes Gift.
    /// The soft descriptor is 800-123-1234.
    /// Then, the statement descriptor on the card is PAYPAL Janes Gift 80.
    pub soft_descriptor: Option<String>,

    /// An array of items that the customer purchases from the merchant.
    pub items: Vec<Item>,

    /// The name and address of the person to whom to ship the items.
    pub shipping: Option<ShippingDetail>,
}

impl PurchaseUnitRequest {
    pub fn new(amount: AmountWithBreakdown) -> PurchaseUnitRequest {
        PurchaseUnitRequest {
            amount,
            items: vec![],
            reference_ids: None,
            payee: None,
            payment_instruction: None,
            description: None,
            custom_id: None,
            invoice_id: None,
            soft_descriptor: None,
            shipping: None,
        }
    }

    pub fn reference_ids(&mut self, reference_ids: String) -> &mut Self {
        self.reference_ids = Some(reference_ids);
        self
    }

    pub fn amount(&mut self, amount: AmountWithBreakdown) -> &mut Self {
        self.amount = amount;
        self
    }

    pub fn payee(&mut self, payee: Payee) -> &mut Self {
        self.payee = Some(payee);
        self
    }

    pub fn payment_instruction(&mut self, payment_instruction: PaymentInstruction) -> &mut Self {
        self.payment_instruction = Some(payment_instruction);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn custom_id(&mut self, custom_id: String) -> &mut Self {
        self.custom_id = Some(custom_id);
        self
    }

    pub fn invoice_id(&mut self, invoice_id: String) -> &mut Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn soft_descriptor(&mut self, soft_descriptor: String) -> &mut Self {
        self.soft_descriptor = Some(soft_descriptor);
        self
    }

    pub fn items(&mut self, items: Vec<Item>) -> &mut Self {
        self.items = items;
        self
    }

    pub fn shipping(&mut self, shipping: ShippingDetail) -> &mut Self {
        self.shipping = Some(shipping);
        self
    }
}
