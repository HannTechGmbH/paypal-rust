use crate::resources::refund_status_details::RefundStatusDetails;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Refund {
    /// The status of the refund.
    pub status: Option<String>,

    /// The details of the refund status.
    pub status_details: Option<RefundStatusDetails>,
}
