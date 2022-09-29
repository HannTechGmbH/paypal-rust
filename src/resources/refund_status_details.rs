use crate::resources::enums::refund_status_reason::RefundStatusReason;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RefundStatusDetails {
    /// The reason why the refund has the PENDING or FAILED status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundStatusReason>,
}
