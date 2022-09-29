use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    /// The full street address component. Can include house number, street name.
    pub street_address: Option<String>,
    /// The city or locality.
    pub locality: Option<String>,
    /// The state, province, prefecture, or region.
    pub region: Option<String>,
    /// The zip code or postal code.
    pub postal_code: Option<String>,
    /// The country name.
    pub country: Option<String>,
}
