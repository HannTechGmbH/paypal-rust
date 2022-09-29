use crate::resources::card_response::CardResponse;
use crate::resources::paypal_payment_source_response::PayPalPaymentSourceResponse;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentSourceResponse {
    pub card: Option<CardResponse>,

    pub paypal: Option<PayPalPaymentSourceResponse>,
}
