use crate::resources::address::Address;
use crate::resources::date_no_time::DateNoTime;
use crate::resources::name::Name;
use crate::resources::phone_with_type::PhoneWithType;
use crate::resources::tax_info::TaxInfo;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payer {
    /// The email address of the payer.
    pub email_address: Option<String>,

    pub payer_id: Option<String>,

    pub name: Option<Name>,

    pub phone: Option<PhoneWithType>,

    pub birth_date: Option<DateNoTime>,

    pub tax_info: Option<TaxInfo>,

    pub address: Address,
}
