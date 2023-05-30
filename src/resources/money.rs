use serde::{Deserialize, Serialize};

use crate::resources::enums::currency_code::CurrencyCode;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Money {
    /// The three-character ISO-4217 currency code that identifies the currency.
    pub currency_code: CurrencyCode,

    /// The value, which might be:
    /// * An integer for currencies like JPY that are not typically fractional.
    /// * A decimal fraction for currencies like TND that are subdivided into thousandths.
    /// For the required number of decimal places for a currency code, see [Currency Codes](<https://developer.paypal.com/api/rest/reference/currency-codes/>).
    pub value: String,
}

impl Money {
    #[must_use]
    pub const fn new(currency_code: CurrencyCode, value: String) -> Money {
        Self {
            currency_code,
            value,
        }
    }
}
