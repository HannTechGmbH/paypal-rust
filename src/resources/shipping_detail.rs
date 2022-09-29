use crate::resources::enums::shipping_type::ShippingType;
use crate::resources::shipping_detail_address_portable::ShippingDetailAddressPortable;
use crate::resources::shipping_detail_name::ShippingDetailName;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingDetail {
    /// The name of the person to whom to ship the items. Supports only the full_name property.
    pub name: Option<ShippingDetailName>,

    /// The method by which the payer wants to get their items from the payee e.g shipping, in-person pickup.
    /// Either type or options but not both may be present.
    #[serde(rename = "type")]
    pub type_: Option<ShippingType>,

    /// The address of the person to whom to ship the items. Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2,
    /// postal_code, and country_code properties.
    pub address: Option<ShippingDetailAddressPortable>,
}
