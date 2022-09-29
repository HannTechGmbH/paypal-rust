use crate::resources::enums::token_type::TokenType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Token {
    /// The PayPal-generated ID for the token.
    pub id: String,

    ///The tokenization method that generated the ID.
    #[serde(rename = "type")]
    pub type_: TokenType,
}
