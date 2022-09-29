//! # paypal-rust
//! ---
//! [![Downloads](https://img.shields.io/crates/d/paypal-rust?style=for-the-badge)](https://crates.io/crates/paypal-rust)
//! [![Version](https://img.shields.io/crates/v/paypal-rust?style=for-the-badge)](https://crates.io/crates/paypal-rust)
//!
//! Rust bindings for the PayPal [REST](https://developer.paypal.com/api/rest) API.
//!
//! This library is a work in progress. While functional, it is not yet advised
//! to be used in production (The API will change, tests are not yet complete, etc).
//!
//! For more information on the PayPal REST API, see the [PayPal Developer Portal](https://developer.paypal.com/api/rest).
//! We aren't affiliated with PayPal in any way and this library is not endorsed by them.
//!
//! ## Usage
//!
//! ```rust
//! use dotenv::dotenv;
//! use paypal_rust::client::AppInfo;
//! use paypal_rust::{
//!     AmountWithBreakdown, Client, CreateOrderDto, CurrencyCode, Environment, Order,
//!     OrderApplicationContext, OrderIntent, PurchaseUnitRequest
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!     let username = std::env::var("CLIENT_ID")?;
//!     let password = std::env::var("CLIENT_SECRET")?;
//!
//!     let mut client = Client::new(username, password, Environment::Sandbox).with_app_info(AppInfo {
//!         name: "PayPal Rust Test App".to_string(),
//!         version: "1.0".to_string(),
//!         website: None,
//!     });
//!
//!     client.authenticate().await.unwrap();
//!
//!     let order = Order::create(
//!         &mut client,
//!         CreateOrderDto {
//!             intent: OrderIntent::Capture,
//!             payer: None,
//!             purchase_units: vec![PurchaseUnitRequest::new(AmountWithBreakdown::new(
//!                 CurrencyCode::Euro,
//!                 "100.00".to_string(),
//!             ))],
//!             application_context: Some(
//!                 OrderApplicationContext::new()
//!                     .return_url("https://example.com/#/return".to_string())
//!                     .cancel_url("https://example.com/#/cancel".to_string()),
//!             ),
//!         },
//!     ).await.unwrap();
//!
//!     println!("Created order: {:?}", order);
//! }
//! ```

#![forbid(unsafe_code)]

pub mod client;
pub mod resources;

pub use client::paypal::*;
pub use resources::*;
