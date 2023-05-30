use std::thread::sleep;
use std::time::Duration;

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

    let client = Client::new(username, password, Environment::Sandbox)
        .unwrap()
        .with_app_info(&AppInfo {
            name: "PayPal Rust Test App".to_string(),
            version: "1.0".to_string(),
            website: None,
        });

    client.authenticate().await.unwrap();

    let new_order = Order::create(
        &client,
        CreateOrderDto {
            intent: OrderIntent::Capture,
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
    .unwrap();

    println!(
        "Approval URL: {:?}. Waiting 15 seconds before capturing",
        new_order.links.unwrap().get(1).unwrap().href
    );

    sleep(Duration::from_secs(15));

    let capture = Order::capture(&client, &new_order.id.unwrap(), None)
        .await
        .unwrap();

    println!("Capture succeeded!");
    println!("Capture: {:?}", capture);
}
