use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum OrderStatus {
    /// The order was created with the specified context.
    #[serde(rename = "CREATED")]
    Created,
    /// The order was saved and persisted. The order status continues to be in progress until a capture is made with
    /// `final_capture = true` for all purchase units within the order.
    #[serde(rename = "SAVED")]
    Saved,
    /// The customer approved the payment through the PayPal wallet or another form of guest or unbranded payment.
    /// For example, a card, bank account, or so on.
    #[serde(rename = "APPROVED")]
    Approved,
    /// All purchase units in the order are voided.
    #[serde(rename = "VOIDED")]
    Voided,
    /// The payment was authorized or the authorized payment was captured for the order.
    #[serde(rename = "COMPLETED")]
    Completed,
    /// The order requires an action from the payer (e.g. 3DS authentication).
    ///  Redirect the payer to the "rel":"payer-action" HATEOAS link returned as part of the response prior to authorizing or capturing
    ///  the order.
    #[serde(rename = "PAYER_ACTION_REQUIRED")]
    PayerActionRequired,
}

impl OrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "CREATED",
            Self::Saved => "SAVED",
            Self::Approved => "APPROVED",
            Self::Voided => "VOIDED",
            Self::Completed => "COMPLETED",
            Self::PayerActionRequired => "PAYER_ACTION_REQUIRED",
        }
    }
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}

impl FromStr for OrderStatus {
    type Err = OrderStatusError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATED" => Ok(Self::Created),
            "SAVED" => Ok(Self::Saved),
            "APPROVED" => Ok(Self::Approved),
            "VOIDED" => Ok(Self::Voided),
            "COMPLETED" => Ok(Self::Completed),
            "PAYER_ACTION_REQUIRED" => Ok(Self::PayerActionRequired),
            _ => Err(OrderStatusError(())),
        }
    }
}

#[derive(Debug)]
pub struct OrderStatusError(/* private */ ());

impl std::fmt::Display for OrderStatusError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "invalid order status".fmt(formatter)
    }
}

impl std::error::Error for OrderStatusError {
    fn description(&self) -> &str {
        "invalid order status"
    }
}
