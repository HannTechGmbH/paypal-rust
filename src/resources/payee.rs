use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payee {
    /// The email address of merchant.
    pub email_address: String,

    /// The encrypted PayPal account ID of the merchant.
    pub merchant_id: String,
}
