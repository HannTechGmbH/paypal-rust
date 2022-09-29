use serde::{Deserialize, Serialize};

/// The reason why the refund has the PENDING or FAILED status.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum RefundStatusReason {
    /// The customer's account is funded through an eCheck, which has not yet cleared.
    #[serde(rename = "ECHECK")]
    Echeck,
}

impl RefundStatusReason {
    pub fn as_str(self) -> &'static str {
        match self {
            RefundStatusReason::Echeck => "ECHECK",
        }
    }
}

impl AsRef<str> for RefundStatusReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
