use crate::resources::processor_response::ProcessorResponse;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthorizationWithAdditionalData {
    pub processor_response: Option<ProcessorResponse>,

    pub id: Option<String>,
}
