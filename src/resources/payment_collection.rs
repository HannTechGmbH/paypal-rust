use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::resources::authorization_with_additional_data::AuthorizationWithAdditionalData;
use crate::resources::capture::Capture;
use crate::resources::refund::Refund;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentCollection {
    /// An array of authorized payments for a purchase unit. A purchase unit can have zero or more authorized payments.
    pub authorizations: Option<Vec<AuthorizationWithAdditionalData>>,

    /// An array of captured payments for a purchase unit. A purchase unit can have zero or more captured payments.
    pub captures: Option<Vec<Capture>>,

    /// An array of refunds for a purchase unit. A purchase unit can have zero or more refunds.
    pub refunds: Option<Vec<Refund>>,
}
