use dotenv::dotenv;
use paypal_rust::client::AppInfo;
use paypal_rust::{
    AmountWithBreakdown, CaptureAuthorizedPaymentDto, Client, CreateOrderDto, CurrencyCode,
    Environment, Order, OrderApplicationContext, OrderIntent, Payment, PurchaseUnitRequest,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let username = std::env::var("CLIENT_ID").unwrap();
    let password = std::env::var("CLIENT_SECRET").unwrap();

    let client = Client::new(username, password, Environment::Sandbox)
        .unwrap()
        .with_app_info(&AppInfo {
            name: "PayPal Rust Test App".to_string(),
            version: "1.0".to_string(),
            website: None,
        });

    client.authenticate().await.unwrap();

    // Create an order
    let order = Order::create(
        &client,
        CreateOrderDto {
            intent: OrderIntent::Authorize,
            payer: None,
            purchase_units: vec![PurchaseUnitRequest::new(AmountWithBreakdown::new(
                CurrencyCode::Euro,
                "10.00".to_string(),
            ))],
            application_context: Some(
                OrderApplicationContext::new()
                    .return_url("https://example.com/#/return".to_string())
                    .cancel_url("https://example.com/#/cancel".to_string()),
            ),
        },
    )
    .await
    .expect("Failed to create order");

    println!("Order created: {:?}", &order);

    // Wait for the order to be approved with the `approve` link.
    sleep(tokio::time::Duration::from_secs(15)).await;

    // Authorize the order
    let authorized = Order::authorize_payment(&client, &order.id.unwrap())
        .await
        .expect("Failed to authorize order");

    let purchase_units = authorized.purchase_units.unwrap();
    let purchase_unit = purchase_units.first().unwrap();
    let payments = purchase_unit.payments.as_ref().unwrap();
    let payment = &payments.authorizations.as_ref().unwrap()[0];
    let authorization_id = payment.id.as_ref().unwrap();

    // Capture the payment
    let capture = Payment::capture_authorized(
        &client,
        authorization_id.clone(),
        CaptureAuthorizedPaymentDto {
            invoice_id: None,
            note_to_payer: None,
            amount: None,
            is_final_capture: None,
            payment_instruction: None,
            soft_descriptor: None,
        },
    )
    .await
    .unwrap();

    println!("Capture: {:?}", capture);
}
