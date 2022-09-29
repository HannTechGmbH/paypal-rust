use crate::resources::enums::country_codes::CountryCodes;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingDetailAddressPortable {
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub admin_area_2: Option<String>,
    pub admin_area_1: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<CountryCodes>,
}
