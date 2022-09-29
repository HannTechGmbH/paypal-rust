use crate::resources::enums::category::Category;
use crate::resources::money::Money;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Item {
    /// The item name or title.
    pub name: String,

    /// The item price or rate per unit. If you specify unit_amount, purchase_units[].amount.breakdown.item_total is required.
    /// Must equal unit_amount * quantity for all items. unit_amount.value can not be a negative number.
    pub unit_amount: Money,

    /// The item tax for each unit. If tax is specified, purchase_units[].amount.breakdown.tax_total is required.
    /// Must equal tax * quantity for all items. tax.value can not be a negative number.
    pub tax: Option<Money>,

    /// The item quantity. Must be a whole number.
    pub quantity: String,

    /// The detailed item description.
    pub description: Option<String>,

    /// The stock keeping unit (SKU) for the item.
    pub sku: Option<String>,

    /// The item category type.
    pub category: Option<Category>,
}

impl Item {
    pub fn new(name: String, unit_amount: Money, quantity: String) -> Item {
        Item {
            name,
            unit_amount,
            quantity,
            tax: None,
            description: None,
            sku: None,
            category: None,
        }
    }
}
