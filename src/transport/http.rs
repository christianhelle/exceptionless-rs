//! Built-in HTTP transport backed by `reqwest`.
//!
//! This module is an advanced integration surface for callers who need to bring
//! their own configured `reqwest::Client` or inspect how the default wire contract
//! is issued over HTTP.
//!
//! If you copy the examples here into your application, add `reqwest` as a
//! direct dependency in your own `Cargo.toml`.

use async_trait::async_trait;
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};

use super::{SubmissionRequest, SubmissionResult, Transport, TransportError, TransportResponse};

/// Transport implementation that submits event batches with `reqwest`.
///
/// Most applications can use [`Default`] and let the high-level client own this
/// type implicitly. Construct it directly when you need custom TLS, proxies,
/// timeouts, or shared middleware on the underlying `reqwest::Client`.
///
/// # Example
///
/// ```
/// use exceptionless::transport::http::HttpTransport;
///
/// let client = reqwest::Client::builder()
///     .user_agent("my-service/1.0")
///     .build()
///     .unwrap();
///
/// let transport = HttpTransport::new(client);
/// ```
#[derive(Debug, Clone)]
pub struct HttpTransport {
    client: Client,
}

impl HttpTransport {
    /// Wraps an existing `reqwest` client for Exceptionless submissions.
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Default for HttpTransport {
    /// Builds a transport with the crate user agent and default `reqwest` settings.
    fn default() -> Self {
        let client = Client::builder()
            .user_agent(format!("exceptionless-rs/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build reqwest client");
        Self::new(client)
    }
}

#[async_trait]
impl Transport for HttpTransport {
    /// Posts the serialized batch as JSON to the configured Exceptionless endpoint.
    async fn submit_events(
        &self,
        request: SubmissionRequest,
    ) -> Result<SubmissionResult, TransportError> {
        let response = self
            .client
            .post(&request.endpoint)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, request.authorization)
            .body(request.payload)
            .send()
            .await
            .map_err(|error| TransportError::Request(error.to_string()))?;

        let status_code = response.status().as_u16();
        let reason = response.status().canonical_reason().map(ToOwned::to_owned);

        let body = response
            .text()
            .await
            .map_err(|error| TransportError::ResponseBody(error.to_string()))?;

        let message = extract_message(status_code, reason, &body);
        let response = TransportResponse::new(status_code, message);

        Ok(SubmissionResult::from_response(response))
    }
}

fn extract_message(status_code: u16, reason: Option<String>, body: &str) -> Option<String> {
    if (200..=299).contains(&status_code) {
        return None;
    }

    let trimmed = body.trim();
    if !trimmed.is_empty() {
        if trimmed.starts_with('{')
            && let Ok(json) = serde_json::from_str::<serde_json::Value>(trimmed)
            && let Some(message) = json.get("message").and_then(|value| value.as_str())
        {
            return Some(message.to_owned());
        }

        if trimmed.len() < 500 {
            return Some(trimmed.to_owned());
        }
    }

    Some(match reason {
        Some(reason) => format!("{status_code} {reason}"),
        None => status_code.to_string(),
    })
}
