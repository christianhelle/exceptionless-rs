#![cfg(not(feature = "opt-out"))]

mod support;

use exceptionless::{
    config::ClientConfig,
    transport::{
        SubmissionRequest, SubmissionResult, Transport, TransportError,
        retry::RetryingTransport,
    },
};
use support::CapturingTransport;
use std::time::Duration;

#[tokio::test]
async fn success_passes_through_on_first_attempt() {
    let inner = CapturingTransport::success();
    let policy = retry_policies::policies::ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(10), Duration::from_millis(50))
        .build_with_max_retries(2);
    let transport = RetryingTransport::new(inner.clone(), policy);

    let result = submit_dummy(&transport).await.unwrap();
    assert_eq!(result.action, exceptionless::transport::SubmissionAction::Success);

    let requests = inner.requests();
    assert_eq!(requests.len(), 1);
}

async fn submit_dummy(transport: &impl Transport) -> Result<SubmissionResult, TransportError> {
    let config = ClientConfig::new("test-key").with_server_url("https://example.com");
    let request = SubmissionRequest::from_events(&config, &[]).unwrap();
    transport.submit_events(request).await
}
