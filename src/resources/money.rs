use crate::resources::enums::currency_code::CurrencyCode;
use serde::{Deserialize, Serialize};

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
    pub fn new(currency_code: CurrencyCode, value: String) -> Money {
        Money {
            currency_code,
            value,
        }
    }
}
