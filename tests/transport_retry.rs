#![cfg(not(feature = "opt-out"))]

mod support;

use exceptionless::{
    config::ClientConfig,
    transport::{
        SubmissionRequest, SubmissionResult, Transport, TransportError, TransportResponse,
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

#[tokio::test]
async fn discard_action_passes_through() {
    let inner = CapturingTransport::new(SubmissionResult::from_response(
        TransportResponse::new(400, None),
    ));
    let policy = retry_policies::policies::ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(10), Duration::from_millis(50))
        .build_with_max_retries(2);
    let transport = RetryingTransport::new(inner.clone(), policy);

    let result = submit_dummy(&transport).await.unwrap();
    assert_eq!(result.action, exceptionless::transport::SubmissionAction::Discard);

    let requests = inner.requests();
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn split_and_retry_action_passes_through() {
    let inner = CapturingTransport::new(SubmissionResult::from_response(
        TransportResponse::new(413, None),
    ));
    let policy = retry_policies::policies::ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(10), Duration::from_millis(50))
        .build_with_max_retries(2);
    let transport = RetryingTransport::new(inner.clone(), policy);

    let result = submit_dummy(&transport).await.unwrap();
    assert_eq!(
        result.action,
        exceptionless::transport::SubmissionAction::SplitAndRetry
    );

    let requests = inner.requests();
    assert_eq!(requests.len(), 1);
}

async fn submit_dummy(transport: &impl Transport) -> Result<SubmissionResult, TransportError> {
    let config = ClientConfig::new("test-key").with_server_url("https://example.com");
    let request = SubmissionRequest::from_events(&config, &[]).unwrap();
    transport.submit_events(request).await
}
