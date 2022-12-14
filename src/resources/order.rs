use crate::client::endpoint::{EmptyResponseBody, Endpoint};
use crate::client::error::PayPalError;
use crate::client::paypal::Client;
use crate::resources::enums::order_intent::OrderIntent;
use crate::resources::enums::order_status::OrderStatus;
use crate::resources::enums::processing_instruction::ProcessingInstruction;
use crate::resources::link_description::LinkDescription;
use crate::resources::order_application_context::OrderApplicationContext;
use crate::resources::patch::Patch;
use crate::resources::payer::Payer;
use crate::resources::payment_source::PaymentSource;
use crate::resources::payment_source_response::PaymentSourceResponse;
use crate::resources::purchase_unit::PurchaseUnit;
use crate::resources::purchase_unit_request::PurchaseUnitRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Order {
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<String>,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<String>,

    /// The ID of the order.
    pub id: Option<String>,

    /// The payment source used to fund the payment.
    pub payment_source: Option<PaymentSourceResponse>,

    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Option<OrderIntent>,

    /// The customer who approves and pays for the order. The customer is also known as the payer.
    #[deprecated]
    pub payer: Option<Payer>,

    /// An array of purchase units. Each purchase unit establishes a contract between a customer and merchant. Each purchase unit
    /// represents either a full or partial order that the customer intends to purchase from the merchant.
    pub purchase_units: Option<Vec<PurchaseUnit>>,

    /// The instruction to process an order.
    pub processing_instruction: Option<ProcessingInstruction>,

    /// The order status.
    pub status: Option<OrderStatus>,

    /// An array of request-related HATEOAS links. To complete payer approval, use the approve link to redirect the payer.
    /// The API caller has 3 hours (default setting, this which can be changed by your account manager to
    /// 24/48/72 hours to accommodate your use case) from the time the order is created, to redirect your payer.
    /// Once redirected, the API caller has 3 hours for the payer to approve the order and either authorize or capture the order.
    /// If you are not using the PayPal JavaScript SDK to initiate PayPal Checkout (in context) ensure that you include
    /// application_context.return_url is specified or you will get "We're sorry, Things don't appear to be working at the moment"
    /// after the payer approves the payment.
    pub links: Option<Vec<LinkDescription>>,
}

impl Order {
    /// Creates an order.
    pub async fn create(client: &mut Client, dto: CreateOrderDto) -> Result<Order, PayPalError> {
        client.post(&CreateOrder::new(dto)).await
    }

    /// Shows details for an order, by ID.
    pub async fn show_details(client: &mut Client, id: &str) -> Result<Order, PayPalError> {
        client.get(&ShowOrderDetails::new(id.to_string())).await
    }

    /// Updates an order with a CREATED or APPROVED status. You cannot update an order with the COMPLETED status.
    ///
    /// To make an update, you must provide a reference_id. If you omit this value with an order
    /// that contains only one purchase unit, PayPal sets the value to default which enables you to
    /// use the path: "/purchase_units/@reference_id=='default'/{attribute-or-object}"
    pub async fn patch(
        client: &mut Client,
        id: &str,
        dto: PatchOrderDto,
    ) -> Result<(), PayPalError> {
        client.patch(&PatchOrder::new(id.to_string(), dto)).await
    }

    /// Authorizes payment for an order. To successfully authorize payment for an order, the buyer
    /// must first approve the order or a valid payment_source must be provided in the request.
    /// A buyer can approve the order upon being redirected to the rel:approve URL that was returned
    /// in the HATEOAS links in the create order response.
    pub async fn authorize_payment(
        client: &mut Client,
        id: &str,
    ) -> Result<AuthorizePaymentForOrderResponse, PayPalError> {
        client
            .post(&AuthorizePaymentForOrder::new(id.to_string()))
            .await
    }

    /// Captures payment for an order. To successfully capture payment for an order,
    /// the buyer must first approve the order or a valid payment_source must be provided in the
    /// request. A buyer can approve the order upon being redirected to the rel:approve URL that
    /// was returned in the HATEOAS links in the create order response.
    pub async fn capture(
        client: &mut Client,
        id: &str,
        payment_source: Option<PaymentSource>,
    ) -> Result<CapturePaymentForOrderResponse, PayPalError> {
        client
            .post(&CapturePaymentForOrder {
                order_id: id.to_string(),
                payment_source,
            })
            .await
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CreateOrderDto {
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    ///
    /// The possible values are:
    /// - CAPTURE. The merchant intends to capture payment immediately after the customer makes a payment.
    /// - AUTHORIZE. The merchant intends to authorize a payment and place funds on hold after the customer makes a payment.
    ///  Authorized payments are best captured within three days of authorization but are available to capture for up to 29 days.
    ///  After the three-day honor period, the original authorized payment expires and you must re-authorize the payment.
    ///  You must make a separate request to capture payments on demand. This intent is not supported when you have more than one
    ///  `purchase_unit` within your order.
    pub intent: OrderIntent,

    /// The customer who approves and pays for the order. The customer is also known as the payer.
    pub payer: Option<Payer>,

    /// An array of purchase units. Each purchase unit establishes a contract between a payer and the payee. Each purchase unit represents
    /// either a full or partial order that the payer intends to purchase from the payee.
    pub purchase_units: Vec<PurchaseUnitRequest>,

    /// Customize the payer experience during the approval process for the payment with PayPal.
    pub application_context: Option<OrderApplicationContext>,
}

#[derive(Debug)]
struct CreateOrder {
    pub order: CreateOrderDto,
}

impl CreateOrder {
    pub fn new(order: CreateOrderDto) -> Self {
        Self { order }
    }
}

impl Endpoint for CreateOrder {
    type QueryParams = ();
    type RequestBody = CreateOrderDto;
    type ResponseBody = Order;

    fn path(&self) -> Cow<str> {
        Cow::Borrowed("v2/checkout/orders")
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.order.clone())
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

#[derive(Debug)]
struct ShowOrderDetails {
    /// The ID of the order for which to show details.
    order_id: String,
}

impl ShowOrderDetails {
    pub fn new(order_id: String) -> Self {
        Self { order_id }
    }
}

impl Endpoint for ShowOrderDetails {
    type QueryParams = ();
    type RequestBody = ();
    type ResponseBody = Order;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v2/checkout/orders/{}", self.order_id))
    }
}

#[derive(Debug)]
pub struct PatchOrderDto {
    pub patch: Vec<Patch>,
}

type PatchOrderResponse = EmptyResponseBody;

#[derive(Debug)]
pub struct PatchOrder {
    order_id: String,
    order: PatchOrderDto,
}

impl PatchOrder {
    pub fn new(order_id: String, order: PatchOrderDto) -> Self {
        Self { order_id, order }
    }
}

impl Endpoint for PatchOrder {
    type QueryParams = ();
    type RequestBody = Vec<Patch>;
    type ResponseBody = PatchOrderResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v2/checkout/orders/{}", self.order_id))
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.order.patch.clone())
    }

    fn request_method(&self) -> Method {
        Method::PATCH
    }
}

/// Authorizes payment for an order. To successfully authorize payment for an order, the buyer must
/// first approve the order or a valid payment_source must be provided in the request.
/// A buyer can approve the order upon being redirected to the rel:approve URL that was returned in
/// the HATEOAS links in the create order response.
#[derive(Debug)]
struct AuthorizePaymentForOrder {
    /// The ID of the order for which to authorize.
    order_id: String,
}

impl AuthorizePaymentForOrder {
    pub fn new(order_id: String) -> Self {
        Self { order_id }
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Default)]
pub struct AuthorizePaymentForOrderResponse {
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<String>,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub update_time: Option<String>,

    /// The ID of the order.
    pub id: Option<String>,

    /// The payment source used to fund the payment.
    pub payment_source: Option<PaymentSourceResponse>,

    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Option<OrderIntent>,

    /// The customer who approves and pays for the order. The customer is also known as the payer.
    pub payer: Option<Payer>,

    /// An array of purchase units. Each purchase unit establishes a contract between a customer and merchant. Each purchase unit
    /// represents either a full or partial order that the customer intends to purchase from the merchant.
    pub purchase_units: Option<Vec<PurchaseUnit>>,

    /// The order status.r.
    pub status: Option<OrderStatus>,

    /// An array of request-related HATEOAS links. To complete payer approval, use the approve link to redirect the payer.
    /// The API caller has 3 hours (default setting, this which can be changed by your account manager to
    /// 24/48/72 hours to accommodate your use case) from the time the order is created, to redirect your payer.
    /// Once redirected, the API caller has 3 hours for the payer to approve the order and either authorize or capture the order.
    /// If you are not using the PayPal JavaScript SDK to initiate PayPal Checkout (in context) ensure that you include
    /// application_context.return_url is specified or you will get "We're sorry, Things don't appear to be working at the moment"
    /// after the payer approves the payment.
    pub links: Option<Vec<LinkDescription>>,
}

impl Endpoint for AuthorizePaymentForOrder {
    type QueryParams = ();
    type RequestBody = ();
    type ResponseBody = AuthorizePaymentForOrderResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v2/checkout/orders/{}/authorize", self.order_id))
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}

struct CapturePaymentForOrder {
    /// The ID of the order for which to capture.
    order_id: String,

    /// The payment source definition
    payment_source: Option<PaymentSource>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Default)]
pub struct CapturePaymentForOrderResponse {
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<String>,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub update_time: Option<String>,

    /// The ID of the order.
    pub id: Option<String>,

    /// The payment source used to fund the payment.
    pub payment_source: Option<PaymentSourceResponse>,

    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Option<OrderIntent>,

    /// The customer who approves and pays for the order. The customer is also known as the payer.
    pub payer: Option<Payer>,

    /// An array of purchase units. Each purchase unit establishes a contract between a customer and merchant. Each purchase unit
    /// represents either a full or partial order that the customer intends to purchase from the merchant.
    pub purchase_units: Option<Vec<PurchaseUnit>>,

    /// The order status.r.
    pub status: Option<OrderStatus>,

    /// An array of request-related HATEOAS links. To complete payer approval, use the approve link to redirect the payer.
    /// The API caller has 3 hours (default setting, this which can be changed by your account manager to
    /// 24/48/72 hours to accommodate your use case) from the time the order is created, to redirect your payer.
    /// Once redirected, the API caller has 3 hours for the payer to approve the order and either authorize or capture the order.
    /// If you are not using the PayPal JavaScript SDK to initiate PayPal Checkout (in context) ensure that you include
    /// application_context.return_url is specified or you will get "We're sorry, Things don't appear to be working at the moment"
    /// after the payer approves the payment.
    pub links: Option<Vec<LinkDescription>>,
}

impl Endpoint for CapturePaymentForOrder {
    type QueryParams = ();
    type RequestBody = Option<PaymentSource>;
    type ResponseBody = CapturePaymentForOrderResponse;

    fn path(&self) -> Cow<str> {
        Cow::Owned(format!("v2/checkout/orders/{}/capture", self.order_id))
    }

    fn request_body(&self) -> Option<Self::RequestBody> {
        Some(self.payment_source.clone())
    }

    fn request_method(&self) -> Method {
        Method::POST
    }
}
