use dotenv::dotenv;
use paypal_rust::client::AppInfo;
use paypal_rust::{
    AmountWithBreakdown, Client, CreateOrderDto, CurrencyCode, Environment, Order,
    OrderApplicationContext, OrderIntent, PurchaseUnitRequest,
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

    let new_order = Order::create(
        &mut client,
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
    .unwrap();

    println!("{:?}", new_order);
}
