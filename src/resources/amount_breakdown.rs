use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::resources::money::Money;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AmountBreakdown {
    /// The subtotal for all items. Required if the request includes purchase_units[].items[].unit_amount.
    /// Must equal the sum of (items[].unit_amount * items[].quantity) for all items. item_total.value can not be a negative number.
    pub item_total: Option<Money>,

    /// The shipping fee for all items within a given purchase_unit. shipping.value can not be a negative number.
    pub shipping: Option<Money>,

    /// The handling fee for all items within a given purchase_unit. handling.value can not be a negative number.
    pub handling: Option<Money>,

    /// The total tax for all items. Required if the request includes purchase_units.items.tax.
    /// Must equal the sum of (items[].tax * items[].quantity) for all items. tax_total.value can not be a negative number.
    pub tax_total: Option<Money>,

    /// The insurance fee for all items within a given purchase_unit. insurance.value can not be a negative number.
    pub insurance: Option<Money>,

    /// The shipping discount for all items within a given purchase_unit. shipping_discount.value can not be a negative number.
    pub shipping_discount: Option<Money>,

    /// The discount for all items within a given purchase_unit. discount.value can not be a negative number.
    pub discount: Option<Money>,
}

impl AmountBreakdown {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            item_total: None,
            shipping: None,
            handling: None,
            tax_total: None,
            insurance: None,
            shipping_discount: None,
            discount: None,
        }
    }

    #[must_use]
    pub fn item_total(mut self, item_total: Money) -> Self {
        self.item_total = Some(item_total);
        self
    }

    #[must_use]
    pub fn shipping(mut self, shipping: Money) -> Self {
        self.shipping = Some(shipping);
        self
    }

    #[must_use]
    pub fn handling(mut self, handling: Money) -> Self {
        self.handling = Some(handling);
        self
    }

    #[must_use]
    pub fn tax_total(mut self, tax_total: Money) -> Self {
        self.tax_total = Some(tax_total);
        self
    }

    #[must_use]
    pub fn insurance(mut self, insurance: Money) -> Self {
        self.insurance = Some(insurance);
        self
    }

    #[must_use]
    pub fn shipping_discount(mut self, shipping_discount: Money) -> Self {
        self.shipping_discount = Some(shipping_discount);
        self
    }

    #[must_use]
    pub fn discount(mut self, discount: Money) -> Self {
        self.discount = Some(discount);
        self
    }
}
