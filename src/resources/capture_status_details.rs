use crate::resources::enums::capture_status_reason::CaptureStatusReason;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CaptureStatusDetails {
    /// The reason why the captured payment status is PENDING or DENIED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<CaptureStatusReason>,
}
