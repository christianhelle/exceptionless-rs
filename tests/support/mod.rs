use std::sync::{Arc, Mutex};

use async_trait::async_trait;
#[cfg(not(feature = "opt-out"))]
use exceptionless::config::ClientConfig;
use exceptionless::transport::{
    SubmissionRequest, SubmissionResult, Transport, TransportError, TransportResponse,
};
#[cfg(not(feature = "opt-out"))]
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct CapturingTransport {
    requests: Arc<Mutex<Vec<SubmissionRequest>>>,
    result: SubmissionResult,
}

impl CapturingTransport {
    pub fn success() -> Self {
        Self::new(SubmissionResult::from_response(TransportResponse::new(
            202, None,
        )))
    }

    pub fn new(result: SubmissionResult) -> Self {
        Self {
            requests: Arc::new(Mutex::new(Vec::new())),
            result,
        }
    }

    pub fn requests(&self) -> Vec<SubmissionRequest> {
        self.requests.lock().unwrap().clone()
    }
}

#[async_trait]
impl Transport for CapturingTransport {
    async fn submit_events(
        &self,
        request: SubmissionRequest,
    ) -> Result<SubmissionResult, TransportError> {
        self.requests.lock().unwrap().push(request);
        Ok(self.result.clone())
    }
}

#[cfg(not(feature = "opt-out"))]
#[allow(dead_code)]
pub fn test_config() -> ClientConfig {
    ClientConfig::new("test-api-key").with_server_url("https://example.com")
}

#[cfg(not(feature = "opt-out"))]
#[allow(dead_code)]
pub fn payload_events(request: &SubmissionRequest) -> Vec<Value> {
    serde_json::from_str(&request.payload).expect("request payload should be valid json")
}
