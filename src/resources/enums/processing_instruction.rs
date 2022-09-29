use serde::{Deserialize, Serialize};

/// The instruction to process an order.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ProcessingInstruction {
    /// API Caller expects the Order to be auto completed (i.e. for PayPal to authorize or capture depending on the intent)
    /// on completion of payer approval. This option is not relevant for payment_source that typically do not require a
    /// payer approval or interaction. This option is currently only available for the following payment_source:
    /// Alipay, Bancontact, BLIK, boletobancario, eps, giropay, GrabPay, iDEAL, Multibanco, MyBank, OXXO, P24, PayU, PUI,
    /// SafetyPay, SatisPay, Sofort, Trustly, Verkkopankki, WeChat Pay
    #[serde(rename = "ORDER_COMPLETE_ON_PAYMENT_APPROVAL")]
    OrderCompleteOnPaymentApproval,
    /// The API caller intends to authorize v2/checkout/orders/id/authorize or capture
    /// v2/checkout/orders/id/capture after the payer approves the order.
    #[serde(rename = "NO_INSTRUCTION")]
    NoInstruction,
}

impl ProcessingInstruction {
    pub fn as_str(self) -> &'static str {
        match self {
            ProcessingInstruction::OrderCompleteOnPaymentApproval => {
                "ORDER_COMPLETE_ON_PAYMENT_APPROVAL"
            }
            ProcessingInstruction::NoInstruction => "NO_INSTRUCTION",
        }
    }
}

impl AsRef<str> for ProcessingInstruction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ProcessingInstruction {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
