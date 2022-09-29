use crate::resources::enums::shipping_type::ShippingType;
use crate::resources::money::Money;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingOption {
    ///  A unique ID that identifies a payer-selected shipping option.
    pub id: String,

    /// A description that the payer sees, which helps them choose an appropriate shipping option. For example, Free Shipping,
    /// USPS Priority Shipping, Expédition prioritaire USPS, or USPS yōuxiān fā huò. Localize this description to the payer's locale.
    pub label: String,

    /// The method by which the payer wants to get their items.
    #[serde(rename = "type")]
    pub type_: Option<ShippingType>,

    ///  The shipping cost for the selected option.
    pub amount: Option<Money>,

    /// If the API request sets selected = true, it represents the shipping option that the payee or merchant expects to be pre-selected
    /// for the payer when they first view the shipping.options in the PayPal Checkout experience. As part of the response if a
    /// `shipping.option` contains selected=true, it represents the shippingoption that the payer selected during the course of checkout with
    /// PayPal. Only one shipping.option can be set to selected=true.
    pub selected: bool,
}
