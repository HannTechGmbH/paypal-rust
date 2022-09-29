use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeRate {
    /// The source currency from which to convert an amount.
    pub source_currency: Option<String>,

    /// The target currency to which to convert an amount.
    pub target_currency: Option<String>,

    // The target currency amount. Equivalent to one unit of the source currency. Formatted as integer or decimal value with one to
    // 15 digits to the right of the decimal point.
    pub value: Option<String>,
}
