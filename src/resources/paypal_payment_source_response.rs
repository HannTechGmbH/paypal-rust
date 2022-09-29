use crate::resources::enums::country_codes::CountryCodes;
use crate::resources::name::Name;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayPalPaymentSourceResponse {
    pub account_id: Option<String>,

    pub address: Option<PayPalPaymentSourceResponseAddress>,

    pub email_address: Option<String>,

    pub name: Option<Name>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayPalPaymentSourceResponseAddress {
    pub country_code: CountryCodes,
}
