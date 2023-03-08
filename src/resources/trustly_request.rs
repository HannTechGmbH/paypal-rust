use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ExperienceContextBase;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TrustlyRequest {
    /// The two-character ISO 3166-1 country code.
    pub country_code: String,

    /// The name of the account holder associated with this payment method.
    pub name: String,

    /// Customizes the payer experience during the approval process for the payment.
    pub experience_context: Option<ExperienceContextBase>,
}
