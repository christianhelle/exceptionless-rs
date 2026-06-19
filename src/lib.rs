//! # Exceptionless for Rust
//!
//! `exceptionless` is an async client for the first Exceptionless Rust SDK slice:
//! error reporting, structured logs, and feature usage events.
//!
//! The main path is [`ExceptionlessClient`]. In a typical application you:
//!
//! 1. create a client with [`ExceptionlessClient::with_api_key`],
//! 2. choose [`ExceptionlessClient::capture_error`], [`ExceptionlessClient::log`], or
//!    [`ExceptionlessClient::feature`],
//! 3. call `send().await`, then inspect the returned
//!    [`transport::SubmissionResult`].
//!
//! This crate targets the hosted Exceptionless collector by default and can also
//! send to a self-hosted server.
//!
//! ## Add it to your application
//!
//! ```toml
//! [dependencies]
//! exceptionless = "0.1"
//! tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
//! ```
//!
//! The examples below use `tokio` because it is the easiest copy-paste setup for
//! a first run on docs.rs. Any async runtime can drive the futures returned by
//! the client APIs.
//!
//! ## First event: start with `ExceptionlessClient`
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
//!         .data("region", "eu-west")
//!         .send()
//!         .await?;
//!
//!     if result.response.is_success() {
//!         println!("event accepted");
//!     } else {
//!         eprintln!("event not accepted: {:?}", result.action);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! If you only remember one flow, remember that one: start with
//! [`ExceptionlessClient`], build the event inline, and inspect the submission
//! result when you care about delivery outcomes.
//!
//! ## Choose the builder that matches the job
//!
//! ### Report an error
//!
//! Use [`ExceptionlessClient::capture_error`] when you already have an error value and
//! want Exceptionless to capture its message, chain, and current stack frames.
//!
//! ```no_run
//! use exceptionless::ExceptionlessClient;
//! use std::io;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
//!     let error = io::Error::other("disk is full");
//!
//!     client
//!         .capture_error(&error)
//!         .source("billing-import")
//!         .tag("storage")
//!         .data("tenant", "acme")
//!         .user_identity("user@example.com")
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Send a structured log
//!
//! Use [`ExceptionlessClient::log`] for application messages that are useful even
//! when no Rust error exists.
//!
//! ```no_run
//! use exceptionless::ExceptionlessClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
//!
//!     client
//!         .log("checkout completed")
//!         .source("payments")
//!         .level("info")
//!         .tag("orders")
//!         .data("order_id", 42)
//!         .version("1.2.3")
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Track feature usage
//!
//! Use [`ExceptionlessClient::feature`] when you want lightweight product usage
//! signals without introducing a second analytics SDK.
//!
//! ```no_run
//! use exceptionless::ExceptionlessClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
//!
//!     client
//!         .feature("export-to-csv")
//!         .tag("reporting")
//!         .user_identity("user@example.com")
//!         .version("1.2.3")
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Self-hosted Exceptionless
//!
//! [`ExceptionlessClient::with_api_key`] targets the hosted collector. For a
//! self-hosted server, build a [`config::ClientConfig`] and pass it to
//! [`ExceptionlessClient::new`].
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
//! ## Advanced extension points
//!
//! Most users only need [`ExceptionlessClient`] plus the default built-in HTTP
//! transport. If you need more control:
//!
//! - [`ExceptionlessClient::new`] accepts any [`transport::Transport`]
//!   implementation.
//! - [`transport`] contains the submission abstractions and the built-in
//!   [`transport::http::HttpTransport`].
//! - [`event::Event`] is the ergonomic public event wrapper for direct submission.
//! - [`wire`] contains the exact low-level payload model used by the transport
//!   layer and is mainly useful for advanced integrations, custom transports, or
//!   tests.
//!
//! Some advanced examples use `reqwest`, `async-trait`, or `serde_json`.
//! Docs.rs can compile those snippets because this crate depends on them, but
//! downstream applications should add those crates explicitly before copying the
//! advanced examples into their own code.
//!
//! ## Retry & Backoff
//!
//! The built-in transport can be wrapped with automatic retry using exponential
//! jittered backoff. This handles transient network errors (connection refused,
//! DNS failures) and retryable HTTP responses (408, 429, 5xx).
//!
//! The [`transport::retry::RetryingTransport`] decorator wraps any
//! [`transport::Transport`] implementation. Construct one with a
//! [`transport::retry::ExponentialBackoff`] policy:
//!
//! ```rust
//! use std::time::Duration;
//! use exceptionless::ExceptionlessClient;
//! use exceptionless::transport::http::HttpTransport;
//! use exceptionless::transport::retry::{ExponentialBackoff, Jitter, RetryingTransport};
//!
//! let policy = ExponentialBackoff::builder()
//!     .retry_bounds(Duration::from_millis(200), Duration::from_secs(10))
//!     .jitter(Jitter::Full)
//!     .base(2)
//!     .build_with_max_retries(2);
//!
//! let transport = RetryingTransport::new(HttpTransport::default(), policy);
//! let client = ExceptionlessClient::new(
//!     exceptionless::config::ClientConfig::new("API_KEY"),
//!     transport,
//! );
//! ```
//!
//! For the quickest on-ramp, use [`ExceptionlessClient::with_api_key_and_retry`]
//! which applies a sensible default policy (3 attempts, 200 ms – 10 s, full jitter):
//!
//! ```rust
//! use exceptionless::ExceptionlessClient;
//!
//! let client = ExceptionlessClient::with_api_key_and_retry("YOUR_API_KEY");
//! ```
//!
//! ## Supported today
//!
//! - Error events with captured stack frames and inner error chaining
//! - Log events with optional source, level, tags, user identity, version, and
//!   custom data
//! - Feature usage events with tags, user identity, version, and custom data
//! - Direct async submission to `POST /api/v2/events`
//! - Hosted Exceptionless or a custom self-hosted server
//! - Custom delivery through the [`transport::Transport`] trait
//! - Automatic retry with exponential jittered backoff via
//!   [`transport::retry::RetryingTransport`] or the
//!   [`ExceptionlessClient::with_api_key_and_retry`] convenience constructor
//!
//! ## Out of scope for this crate today
//!
//! - Automatic queueing or offline storage
//! - Server-side settings or configuration synchronization
//! - Session tracking
//! - Plugin hooks
//! - Logging facade integrations
//!
//! ## Lightweight caveats
//!
//! - `send()` performs direct async submission; there is no background worker yet.
//! - Disabling [`config::ClientConfig`] with [`config::ClientConfig::with_enabled`]
//!   makes submissions fail fast before the transport is invoked.
//! - Enabling the `opt-out` Cargo feature makes `send()` and
//!   [`ExceptionlessClient::submit_batch`] return a synthetic accepted result
//!   without invoking the transport.
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
