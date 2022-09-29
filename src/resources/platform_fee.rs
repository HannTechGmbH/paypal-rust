use crate::resources::money::Money;
use crate::resources::payee_base::PayeeBase;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlatformFee {
    /// The fee for this transaction.
    pub amount: Money,

    /// The recipient of the fee for this transaction. If you omit this value, the default is the API caller.
    pub payee: PayeeBase,
}
