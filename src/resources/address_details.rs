use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AddressDetails {
    /// The street number.
    pub street_number: Option<String>,

    /// The street name. Just "Drury" in "Drury Lane".
    pub street_name: Option<String>,

    /// The street type. For example, avenue, boulevard, road, or expressway.
    pub street_type: Option<String>,

    /// The delivery service. Post office box, bag number, or post office name.
    pub delivery_service: Option<String>,

    /// A named locations that represents the premise. Usually a building name or number or collection of buildings with a common
    /// name or number. For example, Craven House.
    pub building_name: Option<String>,

    /// The first-order entity below a named building or location that represents the sub-premise. Usually a single building within
    ///  a collection of buildings with a common name. Can be a flat, story, floor, room, or apartment.
    pub sub_building: Option<String>,
}
