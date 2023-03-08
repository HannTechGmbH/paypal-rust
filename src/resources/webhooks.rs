use std::borrow::Cow;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::endpoint::Endpoint;
use crate::client::error::PayPalError;
use crate::client::paypal::Client;
use crate::resources::enums::verification_status::VerificationStatus;

pub struct Webhook;

impl Webhook {
    /// Verifies a webhook signature.
    pub async fn verify(
        client: &Client,
        dto: VerifyWebhookSignatureDto,
    ) -> Result<VerifyWebhookSignatureResponse, PayPalError> {
        client.post(&VerifyWebhookSignature::new(dto)).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct VerifyWebhookSignatureDto {
    /// The algorithm that PayPal uses to generate the signature and that you can use to verify the signature.
    /// Extract this value from the `PAYPAL-AUTH-ALGO` response header, which is received with the webhook notification.
    pub auth_algo: String,

    /// The X.509 public key certificate. Download the certificate from this URL and use it to verify the signature.
    /// Extract this value from the `PAYPAL-CERT-URL` response header, which is received with the webhook notification.
    pub cert_url: String,

    /// The ID of the HTTP transmission. Contained in the `PAYPAL-TRANSMISSION-ID` header of the notification message.
    pub transmission_id: String,

    /// The PayPal-generated asymmetric signature. Appears in the `PAYPAL-TRANSMISSION-SIG` header of the notification message.
    pub transmission_sig: String,

    /// The date and time of the HTTP transmission, in Internet date and time format.
    /// Appears in the `PAYPAL-TRANSMISSION-TIME` header of the notification message.
    pub transmission_time: String,

    /// A webhook event notification.
    /// @Note: In this case, the request body.
    pub webhook_event: serde_json::Value,

    /// The ID of the webhook as configured in your Developer Portal account.
    pub webhook_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VerifyWebhookSignatureResponse {
    /// The status of the signature verification.
    pub verification_status: VerificationStatus,
}

#[derive(Debug)]
struct VerifyWebhookSignature {
    pub body: VerifyWebhookSignatureDto,
}

impl VerifyWebhookSignature {
    pub fn new(body: VerifyWebhookSignatureDto) -> Self {
        Self { body }
    }
}

impl Endpoint for VerifyWebhookSignature {
    type QueryParams = ();
    type RequestBody = VerifyWebhookSignatureDto;
    type ResponseBody = VerifyWebhookSignatureResponse;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v1/notifications/verify-webhook-signature")
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.body.clone())
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}
