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
//!     let username = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
//!     let password = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
//!
//!     let mut client = Client::new(username, password, Environment::Sandbox)
//!         .unwrap()
//!         .with_app_info(AppInfo {
//!             name: "PayPal Rust Test App".to_string(),
//!             version: "1.0".to_string(),
//!             website: None,
//!         });
//!
//!     client.authenticate().await.unwrap();
//!
//!     let order = Order::create(
//!         &client,
//!         CreateOrderDto {
//!             intent: OrderIntent::Capture,
//!             payer: None,
//!             purchase_units: vec![PurchaseUnitRequest::new(AmountWithBreakdown::new(
//!                 CurrencyCode::Euro,
//!                 "10.00".to_string(),
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
//!
//! # Features
//!
//! This library offers a "utils" feature that enables the `utils` module. This module contains
//! some useful functions for working with the PayPal API. As of now:
//!
//! - `Order::get_maximum_reauthorization_amount()`
//! - `Order::get_authorization_id()`

#![forbid(unsafe_code)]

pub mod client;
pub mod resources;

pub use client::paypal::*;
pub use resources::*;

#[cfg(feature = "utils")]
pub mod utils;
