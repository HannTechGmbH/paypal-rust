use crate::resources::exchange_rate::ExchangeRate;
use crate::resources::money::Money;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NetAmountBreakdown {
    /// The converted payable amount.
    pub converted_amount: Option<Money>,

    /// The exchange rate that determines the amount that was debited from the merchant's PayPal account.
    pub exchange_rate: Option<ExchangeRate>,

    /// The net amount debited from the merchant's PayPal account.
    pub payable_amount: Option<Money>,
}
