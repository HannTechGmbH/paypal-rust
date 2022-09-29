use dotenv::dotenv;
use paypal_rust::client::AppInfo;
use paypal_rust::{Client, Environment, VerifyWebhookSignatureDto, Webhook};

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

    let verification_status = Webhook::verify(&mut client, VerifyWebhookSignatureDto {
        auth_algo: "SHA256withRSA".to_string(),
        cert_url: "cert_url".to_string(),
        transmission_id: "69cd13f0-d67a-11e5-baa3-778b53f4ae55".to_string(),
        transmission_sig: "lmI95Jx3Y9nhR5SJWlHVIWpg4AgFk7n9bCHSRxbrd8A9zrhdu2rMyFrmz+Zjh3s3boXB07VXCXUZy/UFzUlnGJn0wDugt7FlSvdKeIJenLRemUxYCPVoEZzg9VFNqOa48gMkvF+XTpxBeUx/kWy6B5cp7GkT2+pOowfRK7OaynuxUoKW3JcMWw272VKjLTtTAShncla7tGF+55rxyt2KNZIIqxNMJ48RDZheGU5w1npu9dZHnPgTXB9iomeVRoD8O/jhRpnKsGrDschyNdkeh81BJJMH4Ctc6lnCCquoP/GzCzz33MMsNdid7vL/NIWaCsekQpW26FpWPi/tfj8nLA==".to_string(),
        transmission_time: "2016-02-18T20:01:35Z".to_string(),
        webhook_event: "webhookEvent".to_string(),
        webhook_id: "1JE4291016473214C".to_string(),
    }).await.unwrap().verification_status;

    println!("Verification status: {}", verification_status);
}
