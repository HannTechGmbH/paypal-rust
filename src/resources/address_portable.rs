use crate::resources::address_details::AddressDetails;
use crate::CountryCodes;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AddressPortable {
    /// The first line of the address. For example, number or street.
    /// For example, 173 Drury Lane. Required for data entry and compliance and risk checks. Must contain the full address.
    pub address_line_1: Option<String>,

    /// The second line of the address. For example, suite or apartment number.
    pub address_line_2: Option<String>,

    /// The third line of the address, if needed. For example, a street complement for Brazil, direction text, such as next to Walmart,
    /// or a landmark in an Indian address.
    pub address_line_3: Option<String>,

    /// The neighborhood, ward, or district. Smaller than admin_area_level_3 or sub_locality. Value is:
    /// - The postal sorting code for Guernsey and many French territories, such as French Guiana.
    /// - The fine-grained administrative levels in China.
    pub admin_area_4: Option<String>,

    /// A sub-locality, suburb, neighborhood, or district. Smaller than admin_area_level_2. Value is:
    /// - Brazil. Suburb, bairro, or neighborhood.
    /// - India. Sub-locality or district. Street name information is not always available but a sub-locality or district can be
    ///   a very small area.
    pub admin_area_3: Option<String>,

    /// A city, town, or village. Smaller than admin_area_level_1.
    pub admin_area_2: Option<String>,

    /// The highest level sub-division in a country, which is usually a province, state, or ISO-3166-2 subdivision. Format for postal
    /// delivery. For example, CA and not California. Value, by country, is:
    /// - UK. A county.
    /// - US. A state.
    /// - Canada. A province.
    /// - Japan. A prefecture.
    /// - Switzerland. A kanton.
    pub admin_area_1: Option<String>,

    /// The postal code, which is the zip code or equivalent. Typically required for countries with a postal code or an equivalent
    pub postal_code: Option<String>,

    /// The two-character ISO 3166-1 code that identifies the country or region.
    pub country_code: CountryCodes,

    /// The non-portable additional address details that are sometimes needed for compliance, risk, or other scenarios where fine-grain
    /// address information might be needed. Not portable with common third party and open source. Redundant with core fields.
    /// For example, address_portable.address_line_1 is usually a combination of address_details.street_number, street_name, and street_type.
    pub address_details: AddressDetails,
}
