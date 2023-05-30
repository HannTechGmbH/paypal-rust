use serde::{Deserialize, Serialize};

use crate::resources::amount_breakdown::AmountBreakdown;
use crate::resources::enums::currency_code::CurrencyCode;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AmountWithBreakdown {
    /// The three-character ISO-4217 currency code that identifies the currency.
    pub currency_code: String,

    ///  The value, which might be:
    ///  An integer for currencies like JPY that are not typically fractional.
    ///  A decimal fraction for currencies like TND that are subdivided into thousandths.
    pub value: String,

    ///  The breakdown of the amount. Breakdown provides details such as total
    ///  item amount, total tax amount, shipping, handling, insurance, and discounts, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<AmountBreakdown>,
}

impl AmountWithBreakdown {
    #[must_use]
    pub fn new(currency_code: CurrencyCode, value: String) -> Self {
        Self {
            currency_code: currency_code.as_str().to_string(),
            value,
            breakdown: None,
        }
    }

    #[must_use]
    pub fn breakdown(mut self, breakdown: AmountBreakdown) -> Self {
        self.breakdown = Some(breakdown);
        self
    }
}
