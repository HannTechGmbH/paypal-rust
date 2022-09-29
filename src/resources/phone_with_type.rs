use crate::resources::enums::phone_type::PhoneType;
use crate::resources::phone_with_type_phone::PhoneWithTypePhone;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PhoneWithType {
    /// The phone type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<PhoneType>,

    /// The phone number, in its canonical international E.164 numbering plan format. Supports only the national_number property.
    pub phone_number: PhoneWithTypePhone,
}
