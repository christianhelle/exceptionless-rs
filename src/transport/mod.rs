//! Advanced transport extension points for submitting serialized events.
//!
//! Most consumers should stay on the high-level [`crate::ExceptionlessClient`] API.
//! This module exists for callers who need to inspect the exact wire request, swap
//! in a custom HTTP implementation, or classify server responses manually.
//!
//! The examples in this module use `async-trait` because the public
//! [`Transport`] trait is async. Downstream crates copying those snippets should
//! add `async-trait` to their own `Cargo.toml`.

use std::{error::Error as StdError, fmt};

/// Built-in `reqwest` transport for direct HTTP submission.
pub mod http;
/// Response classification helpers used by transport implementations.
pub mod response;

use async_trait::async_trait;
use serde_json::to_string;

use crate::{
    config::{ClientConfig, ConfigError},
    wire::event::Event,
};

pub use response::{SubmissionAction, SubmissionResult, TransportResponse};

/// Fully materialized request sent to the Exceptionless event endpoint.
///
/// This type is an advanced seam for custom transport implementations. It contains
/// the final endpoint, authorization header value, and serialized JSON payload that
/// the high-level client wants to submit.
///
/// # Example
///
/// ```
/// use exceptionless::{
///     config::ClientConfig,
///     transport::SubmissionRequest,
///     wire::event::Event,
/// };
///
/// let config = ClientConfig::new("API_KEY");
/// let request = SubmissionRequest::from_events(&config, &[Event::log("hello")]).unwrap();
///
/// assert_eq!(request.endpoint, "https://collector.exceptionless.io/api/v2/events");
/// assert_eq!(request.authorization, "Bearer API_KEY");
/// assert!(request.payload.starts_with(r#"[{"type":"log""#));
/// assert!(request.payload.contains(r#""message":"hello""#));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionRequest {
    /// Absolute endpoint that receives the event batch.
    pub endpoint: String,
    /// Value for the `Authorization` header.
    pub authorization: String,
    /// JSON array payload containing the serialized batch.
    pub payload: String,
}

impl SubmissionRequest {
    /// Builds a request from validated config and a batch of wire events.
    ///
    /// The API key is trimmed before forming the `Bearer` header, and the events are
    /// serialized as a JSON array in the order provided.
    pub fn from_events(config: &ClientConfig, events: &[Event]) -> Result<Self, TransportError> {
        let endpoint = config.events_url()?;
        let payload = to_string(events)?;

        Ok(Self {
            endpoint,
            authorization: format!("Bearer {}", config.api_key().trim()),
            payload,
        })
    }
}

#[async_trait]
/// Advanced submission trait implemented by transport backends.
///
/// Implement this trait when you need to send Exceptionless payloads through a
/// custom HTTP stack, test double, or environment-specific delivery mechanism.
/// The trait operates on already-shaped [`SubmissionRequest`] values so custom
/// transports do not need to know about event builders.
///
/// # Example
///
/// ```
/// use async_trait::async_trait;
/// use exceptionless::transport::{
///     SubmissionRequest, SubmissionResult, Transport, TransportError, TransportResponse,
/// };
///
/// struct CapturingTransport;
///
/// #[async_trait]
/// impl Transport for CapturingTransport {
///     async fn submit_events(
///         &self,
///         request: SubmissionRequest,
///     ) -> Result<SubmissionResult, TransportError> {
///         assert!(request.authorization.starts_with("Bearer "));
///         Ok(SubmissionResult::from_response(TransportResponse::new(202, None)))
///     }
/// }
/// ```
///
/// If you copy this example into your application, add `async-trait` as a
/// direct dependency in your own crate.
pub trait Transport: Send + Sync {
    /// Sends a pre-built event batch and returns the server classification result.
    async fn submit_events(
        &self,
        request: SubmissionRequest,
    ) -> Result<SubmissionResult, TransportError>;
}

/// Errors produced while preparing or sending low-level transport requests.
#[derive(Debug)]
pub enum TransportError {
    /// The client configuration could not produce a valid submission request.
    InvalidConfiguration(ConfigError),
    /// The event batch could not be serialized into JSON.
    Serialization(serde_json::Error),
    /// The transport failed before a response body could be classified.
    Request(String),
    /// The response body could not be read after the request completed.
    ResponseBody(String),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(error) => write!(f, "{error}"),
            Self::Serialization(error) => {
                write!(f, "failed to serialize event payload: {error}")
            }
            Self::Request(message) => write!(f, "request failed: {message}"),
            Self::ResponseBody(message) => write!(f, "failed to read response body: {message}"),
        }
    }
}

impl StdError for TransportError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidConfiguration(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::Request(_) | Self::ResponseBody(_) => None,
        }
    }
}

impl From<ConfigError> for TransportError {
    fn from(error: ConfigError) -> Self {
        Self::InvalidConfiguration(error)
    }
}

impl From<serde_json::Error> for TransportError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}
