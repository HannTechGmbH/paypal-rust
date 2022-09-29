use crate::resources::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentSource {
    pub token: Token,
}
