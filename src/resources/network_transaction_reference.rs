use crate::resources::enums::network::Network;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NetworkTransactionReference {
    /// Transaction reference id returned by the scheme. For Visa and Amex, this is the "Tran id" field in response. For MasterCard,
    /// this is the "BankNet reference id" field in response. For Discover, this is the "NRID" field in response.
    pub id: String,

    /// The date that the transaction was authorized by the scheme. For MasterCard, this is the "BankNet reference date" field in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// Name of the card network through which the transaction was routed.
    pub network: Network,
}
