use async_trait::async_trait;
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};

use super::{SubmissionRequest, SubmissionResult, Transport, TransportError, TransportResponse};

#[derive(Debug, Clone)]
pub struct HttpTransport {
    client: Client,
}

impl HttpTransport {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Default for HttpTransport {
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
