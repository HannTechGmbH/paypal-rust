use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardAddressPortable {
    /// The first line of the address. For example, number or street. For example, 173 Drury Lane. Required for data entry and compliance
    /// and risk checks. Must contain the full address.
    pub address_line1: Option<String>,

    /// The second line of the address. For example, suite or apartment number. */
    pub address_line2: Option<String>,

    /// A city, town, or village. Smaller than admin_area_level_1. */
    pub admin_area_2: Option<String>,

    /// The highest level sub-division in a country, which is usually a province, state, or ISO-3166-2 subdivision. Format for postal
    /// delivery. For example, CA and not California. Value, by country, is:
    /// - UK. A county.
    /// - US. A state.
    /// - Canada. A province.
    /// - Japan. A prefecture.
    /// - Switzerland. A kanton.
    pub admin_area_1: Option<String>,

    /// The postal code, which is the zip code or equivalent. Typically required
    /// for countries with a postal code or an equivalent.
    pub postal_code: Option<String>,

    /// The two-character ISO 3166-1 code that identifies the country or region. */
    pub country_code: String,
}
