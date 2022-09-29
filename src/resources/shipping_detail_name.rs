use crate::resources::enums::shipping_type::ShippingType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingDetailName {
    /// When the party is a person, the party's full name.
    pub full_name: String,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ShippingType>,
}
