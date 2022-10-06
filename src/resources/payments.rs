use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;

use crate::client::{Client, Endpoint, PayPalError};
use crate::{
    AuthorizationStatusDetails, CaptureStatus, CaptureStatusDetails, DisbursementMode,
    LinkDescription, Money, PaymentInstruction, PaymentStatus, ProcessorResponse, RefundStatus,
    RefundStatusDetails, SellerPayableBreakdown, SellerProtection, SellerReceivableBreakdown,
};

pub struct Payment;

impl Payment {
    /// Captures an authorized payment, by ID.
    pub async fn capture_authorized(
        client: &mut Client,
        authorization_id: String,
        dto: CaptureAuthorizedPaymentDto,
    ) -> Result<CaptureAuthorizedPaymentResponse, PayPalError> {
        client
            .post(&CaptureAuthorizedPayment::new(authorization_id, dto))
            .await
    }

    /// Refunds a captured payment, by ID. For a full refund, include an empty payload in the JSON
    /// request body. For a partial refund, include an amount object in the JSON request body.
    pub async fn refund_captured(
        client: &mut Client,
        capture_id: String,
        dto: RefundCapturedPaymentDto,
    ) -> Result<RefundCapturedPaymentResponse, PayPalError> {
        client
            .post(&RefundCapturedPayment::new(capture_id, dto))
            .await
    }

    /// Reauthorizes an authorized PayPal account payment, by ID. To ensure that funds are still
    /// available, reauthorize a payment after its initial three-day honor period expires. Within
    /// the 29-day authorization period, you can issue multiple re-authorizations after the honor
    /// period expires.
    ///
    /// If 30 days have transpired since the date of the original authorization, you must create an
    /// authorized payment instead of reauthorizing the original authorized payment.
    ///
    /// A reauthorized payment itself has a new honor period of three days.
    ///
    /// You can reauthorize an authorized payment once for up to 115% of the original authorized
    /// amount, not to exceed an increase of $75 USD.
    ///
    /// Supports only the amount request parameter.
    pub async fn reauthorize_authorized(
        client: &mut Client,
        authorization_id: String,
        dto: ReauthorizeAuthorizedPaymentDto,
    ) -> Result<ReauthorizeAuthorizedPaymentResponse, PayPalError> {
        client
            .post(&ReauthorizeAuthorizedPayment::new(authorization_id, dto))
            .await
    }

    /// Voids, or cancels, an authorized payment, by ID. You cannot void an authorized payment that
    /// has been fully captured.
    pub async fn void_authorized(
        client: &mut Client,
        authorization_id: String,
    ) -> Result<(), PayPalError> {
        client
            .post(&VoidAuthorizedPayment::new(authorization_id))
            .await
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Default)]
pub struct CaptureAuthorizedPaymentDto {
    /// The API caller-provided external invoice number for this order. Appears in both the payer's
    /// transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,

    /// An informational note about this settlement. Appears in both the payer's transaction history
    /// and the emails that the payer receives.
    pub note_to_payer: Option<String>,

    /// The amount to capture. To capture a portion of the full authorized amount, specify an amount.
    /// If amount is not specified, the full authorized amount is captured. The amount must be a
    /// positive number and in the same currency as the authorization against which the payment is
    /// being captured.
    pub amount: Option<Money>,

    ///  Indicates whether you can make additional captures against the authorized payment.
    /// Set to true if you do not intend to capture additional payments against the authorization.
    /// Set to false if you intend to capture additional payments against the authorization.
    pub is_final_capture: Option<bool>,

    ///  Any additional payment instructions to be considered during payment processing.
    /// This processing instruction is applicable for Capturing an order or Authorizing an Order.
    pub payment_instruction: Option<PaymentInstruction>,

    /// The payment descriptor on the payer's account statement.
    pub soft_descriptor: Option<String>,
}

impl CaptureAuthorizedPaymentDto {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn invoice_id(mut self, invoice_id: String) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn note_to_payer(mut self, note_to_payer: String) -> Self {
        self.note_to_payer = Some(note_to_payer);
        self
    }

    pub fn amount(mut self, amount: Money) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn is_final_capture(mut self, is_final_capture: bool) -> Self {
        self.is_final_capture = Some(is_final_capture);
        self
    }

    pub fn payment_instruction(mut self, payment_instruction: PaymentInstruction) -> Self {
        self.payment_instruction = Some(payment_instruction);
        self
    }

    pub fn soft_descriptor(mut self, soft_descriptor: String) -> Self {
        self.soft_descriptor = Some(soft_descriptor);
        self
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CaptureAuthorizedPaymentResponse {
    /// The status of the captured payment.
    pub status: Option<CaptureStatus>,

    /// The details of the captured payment status.
    pub status_details: Option<CaptureStatusDetails>,

    /// The PayPal-generated ID for the captured payment.
    pub id: Option<String>,

    /// The amount for this captured payment.
    pub amount: Option<Money>,

    /// The API caller-provided external invoice number for this order. Appears in both the payer's
    /// transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,

    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions
    /// with PayPal transactions. Appears in transaction and settlement reports.
    pub custom_id: Option<String>,

    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: Option<SellerProtection>,

    /// Indicates whether you can make additional captures against the authorized payment. Set to
    /// true if you do not intend to capture additional payments against the authorization.
    /// Set to false if you intend to capture additional payments against the authorization.
    pub final_capture: Option<bool>,

    /// The detailed breakdown of the capture activity. This is not available for transactions
    /// that are in pending state.
    pub seller_receivable_breakdown: Option<SellerReceivableBreakdown>,

    /// The funds that are held on behalf of the merchant.
    pub disbursement_mode: Option<DisbursementMode>,

    /// An array of related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,

    /// An object that provides additional processor information for a direct credit card
    /// transaction.
    pub processor_response: Option<ProcessorResponse>,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<String>,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<String>,
}

#[derive(Debug, Clone)]
struct CaptureAuthorizedPayment {
    authorization_id: String,
    dto: CaptureAuthorizedPaymentDto,
}

impl Endpoint for CaptureAuthorizedPayment {
    type QueryParams = ();
    type RequestBody = CaptureAuthorizedPaymentDto;
    type ResponseBody = CaptureAuthorizedPaymentResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!(
            "v2/payments/authorizations/{}/capture",
            self.authorization_id
        ))
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.dto.clone())
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

impl CaptureAuthorizedPayment {
    fn new(authorization_id: String, dto: CaptureAuthorizedPaymentDto) -> Self {
        Self {
            authorization_id,
            dto,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct RefundCapturedPaymentDto {
    /// The amount to refund. To refund a portion of the captured amount, specify an amount.
    /// If amount is not specified, an amount equal to captured amount - previous refunds is refunded.
    /// The amount must be a positive number and in the same currency as the one in which the payment was captured.
    pub amount: Option<Money>,

    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,

    /// The reason for the refund. Appears in both the payer's transaction history and the emails that the payer receives.
    pub note_to_payer: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize)]
pub struct RefundCapturedPaymentResponse {
    /// The amount that the payee refunded to the payer.
    pub amount: Money,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: String,

    /// The PayPal-generated ID for the refund.
    pub id: String,

    /// The API caller-provided external invoice number for this order.
    ///  Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: String,

    /// An array of related HATEOAS links.
    pub links: Vec<LinkDescription>,

    /// The reason for the refund. Appears in both the payer's transaction history and the emails that the payer receives.
    pub note_to_payer: String,

    /// The breakdown of the refund.
    pub seller_payable_breakdown: SellerPayableBreakdown,

    /// The status of the refund.
    pub status: RefundStatus,

    /// The details of the refund status.
    pub status_details: RefundStatusDetails,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: String,
}

/// Refunds a captured payment, by ID. For a full refund, include an empty payload in the JSON
/// request body. For a partial refund, include an amount object in the JSON request body.
#[derive(Debug)]
struct RefundCapturedPayment {
    capture_id: String,
    amount: Option<Money>,
    invoice_id: Option<String>,
    note_to_payer: Option<String>,
}

impl RefundCapturedPayment {
    pub fn new(capture_id: String, body: RefundCapturedPaymentDto) -> Self {
        Self {
            capture_id,
            amount: body.amount,
            invoice_id: body.invoice_id,
            note_to_payer: body.note_to_payer,
        }
    }
}

impl Endpoint for RefundCapturedPayment {
    type QueryParams = ();
    type RequestBody = RefundCapturedPaymentDto;
    type ResponseBody = RefundCapturedPaymentResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v2/payments/captures/{}/refund", self.capture_id))
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(RefundCapturedPaymentDto {
            amount: self.amount.clone(),
            invoice_id: self.invoice_id.clone(),
            note_to_payer: self.note_to_payer.clone(),
        })
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ReauthorizeAuthorizedPaymentDto {
    /// The amount to reauthorize for an authorized payment.
    pub amount: Option<Money>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize)]
pub struct ReauthorizeAuthorizedPaymentResponse {
    /// The status for the authorized payment.
    pub status: Option<PaymentStatus>,

    /// The details of the authorized order pending status.
    pub status_details: Option<AuthorizationStatusDetails>,

    /// The PayPal-generated ID for the authorized payment.
    pub id: Option<String>,

    /// The amount for this authorized payment.
    pub amount: Option<Money>,

    /// The API caller-provided external invoice number for this order. Appears in both the payer's transaction history and the emails
    /// that the payer receives.
    pub invoice_id: Option<String>,

    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions with PayPal transactions. Appears in
    /// transaction and settlement reports.
    pub custom_id: Option<String>,

    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: Option<SellerProtection>,

    /// The date and time when the authorized payment expires, in Internet date and time format
    pub expiration_time: Option<String>,

    /// An array of related HATEOAS links.  
    pub links: Option<Vec<LinkDescription>>,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<String>,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<String>,
}

/// Reauthorizes an authorized PayPal account payment, by ID. To ensure that funds are still
/// available, reauthorize a payment after its initial three-day honor period expires.
/// Within the 29-day authorization period, you can issue multiple re-authorizations after the honor
/// period expires.
//
// If 30 days have transpired since the date of the original authorization, you must create an
// authorized payment instead of reauthorizing the original authorized payment.
//
// A reauthorized payment itself has a new honor period of three days.
//
// You can reauthorize an authorized payment once for up to 115% of the original authorized amount,
// not to exceed an increase of $75 USD.
//
// Supports only the amount request parameter.
#[derive(Debug)]
struct ReauthorizeAuthorizedPayment {
    authorization_id: String,
    amount: Option<Money>,
}

impl ReauthorizeAuthorizedPayment {
    pub fn new(authorization_id: String, body: ReauthorizeAuthorizedPaymentDto) -> Self {
        Self {
            authorization_id,
            amount: body.amount,
        }
    }
}

impl Endpoint for ReauthorizeAuthorizedPayment {
    type QueryParams = ();
    type RequestBody = ReauthorizeAuthorizedPaymentDto;
    type ResponseBody = ReauthorizeAuthorizedPaymentResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!(
            "v2/payments/authorizations/{}/reauthorize",
            self.authorization_id
        ))
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(ReauthorizeAuthorizedPaymentDto {
            amount: self.amount.clone(),
        })
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

struct VoidAuthorizedPayment {
    authorization_id: String,
}

impl VoidAuthorizedPayment {
    pub fn new(authorization_id: String) -> Self {
        Self { authorization_id }
    }
}

impl Endpoint for VoidAuthorizedPayment {
    type QueryParams = ();
    type RequestBody = ();
    type ResponseBody = ();

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!(
            "v2/payments/authorizations/{}/void",
            self.authorization_id
        ))
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        None
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}
