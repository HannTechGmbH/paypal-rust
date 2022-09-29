use crate::resources::enums::authorization_status_reason::AuthorizationStatusReason;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthorizationStatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<AuthorizationStatusReason>,
}
