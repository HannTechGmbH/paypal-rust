use serde::{Deserialize, Serialize};

/// The tokenization method that generated the ID.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum TokenType {
    /// The PayPal billing agreement ID. References an approved recurring payment for goods or services.
    #[serde(rename = "BILLING_AGREEMENT")]
    BillingAgreement,
}

impl TokenType {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenType::BillingAgreement => "BILLING_AGREEMENT",
        }
    }
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::BillingAgreement
    }
}

impl AsRef<str> for TokenType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
