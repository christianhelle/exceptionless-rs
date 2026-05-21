//! # Exceptionless client for Rust
//!
//! `exceptionless` is an async Rust client for the current Exceptionless MVP slice:
//! error reporting, log events, and feature usage tracking.
//!
//! ## Supported today
//!
//! - Error events with captured stack frames and inner error chaining
//! - Log events with optional source, level, tags, user identity, version, and custom data
//! - Feature usage events with tags, user identity, version, and custom data
//! - Direct async submission to `POST /api/v2/events` with the default built-in HTTP transport
//! - Default hosted collector or a custom self-hosted Exceptionless server
//! - Custom transports through the [`transport::Transport`] trait
//!
//! ## Not yet supported
//!
//! - Automatic queueing, offline storage, or retry workers
//! - Server-side settings/config synchronization
//! - Session tracking
//! - Plugin hooks
//! - Logging facade integrations
//!
//! ## Quick start
//!
//! ```no_run
//! use exceptionless::ExceptionlessClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
//!
//!     let result = client
//!         .log("worker started")
//!         .source("jobs")
//!         .level("info")
//!         .tag("startup")
//!         .send()
//!         .await?;
//!
//!     if !result.response.is_success() {
//!         eprintln!("submission was not accepted: {:?}", result.action);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Common event builders
//!
//! ```no_run
//! use core::num::ParseIntError;
//! use exceptionless::ExceptionlessClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
//!
//!     let parse_error: ParseIntError = "not a number".parse::<i32>().unwrap_err();
//!     client
//!         .error(&parse_error)
//!         .tag("parsing")
//!         .source("user_input")
//!         .data("raw_value", "not a number")
//!         .send()
//!         .await?;
//!
//!     client
//!         .feature("export_to_pdf")
//!         .user_identity("user@example.com")
//!         .version("1.2.3")
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Custom server configuration
//!
//! ```no_run
//! use exceptionless::ExceptionlessClient;
//! use exceptionless::config::ClientConfig;
//! use exceptionless::transport::http::HttpTransport;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ClientConfig::new("YOUR_API_KEY")
//!         .with_server_url("https://your-exceptionless-server.com");
//!
//!     let client = ExceptionlessClient::new(config, HttpTransport::default());
//!     client.log("self-hosted ready").send().await?;
//!     Ok(())
//! }
//! ```
//!
//! The built-in HTTP transport is available in every build. Enabling the `opt-out`
//! Cargo feature makes `send()` and `submit_batch()` return a no-op success without
//! invoking the transport, even if the client config is disabled or otherwise
//! invalid. Without `opt-out`, disabling a [`config::ClientConfig`] with
//! [`config::ClientConfig::with_enabled`] still causes submission to fail before the
//! transport is invoked.
pub mod builder;
pub mod client;
pub mod config;
pub mod error;
pub mod event;
pub mod feature;
pub mod log;
pub mod transport;
pub mod wire;

pub use client::ExceptionlessClient;
pub use error::{ClientError, ErrorEventBuilder};
pub use feature::FeatureUsageBuilder;
pub use log::LogEventBuilder;
