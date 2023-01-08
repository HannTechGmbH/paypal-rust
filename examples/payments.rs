use dotenv::dotenv;
use paypal_rust::client::AppInfo;
use paypal_rust::{
    AmountWithBreakdown, CaptureAuthorizedPaymentDto, Client, CreateOrderDto, CurrencyCode,
    Environment, Order, OrderApplicationContext, OrderIntent, Payment, PurchaseUnitRequest,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let username = std::env::var("CLIENT_ID").unwrap();
    let password = std::env::var("CLIENT_SECRET").unwrap();

    let mut client = Client::new(username, password, Environment::Sandbox).with_app_info(AppInfo {
        name: "PayPal Rust Test App".to_string(),
        version: "1.0".to_string(),
        website: None,
    });

    client.authenticate().await.unwrap();

    // Create an order
    let order = Order::create(
        &client,
        CreateOrderDto {
            intent: OrderIntent::Capture,
            payer: None,
            purchase_units: vec![PurchaseUnitRequest::new(AmountWithBreakdown::new(
                CurrencyCode::Euro,
                "100.00".to_string(),
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
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    // Authorize the order
    let authorized = Order::authorize_payment(&client, &order.id.unwrap())
        .await
        .expect("Failed to authorize order");

    let authorization_id = authorized
        .purchase_units
        .unwrap()
        .get(0)
        .unwrap()
        .payments
        .clone()
        .unwrap()
        .authorizations
        .clone()
        .unwrap()
        .get(0)
        .unwrap()
        .id
        .clone()
        .unwrap();

    // Capture the payment
    let capture = Payment::capture_authorized(
        &client,
        authorization_id,
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
