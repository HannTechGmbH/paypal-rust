use crate::resources::enums::avs_code::AvsCode;
use crate::resources::enums::cvv_code::CvvCode;
use crate::resources::enums::response_code::ResponseCode;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProcessorResponse {
    /// The address verification code for Visa, Discover, Mastercard, or American Express transactions.
    pub avs_code: Option<AvsCode>,

    /// The card verification value code for for Visa, Discover, Mastercard, or American Express.
    pub cvv_code: Option<CvvCode>,

    /// Processor response code for the non-PayPal payment processor errors.
    pub response_code: Option<ResponseCode>,

    /// The declined payment transactions might have payment advice codes. The card networks, like Visa and Mastercard,
    /// return payment advice codes.
    pub payment_advice_code: Option<i32>,
}
