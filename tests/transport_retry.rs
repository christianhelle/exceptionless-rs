#![cfg(not(feature = "opt-out"))]

mod support;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use exceptionless::{
    config::ClientConfig,
    transport::{
        SubmissionRequest, SubmissionResult, Transport, TransportError, TransportResponse,
        retry::RetryingTransport,
    },
};
use support::CapturingTransport;

/// A transport that returns a sequence of results on successive calls.
#[derive(Debug, Clone)]
struct SequenceTransport {
    results: Arc<Vec<TransportResult>>,
    call_count: Arc<AtomicUsize>,
    requests: Arc<std::sync::Mutex<Vec<SubmissionRequest>>>,
}

#[derive(Debug, Clone)]
struct TransportResult {
    submission: SubmissionResult,
}

impl SequenceTransport {
    fn new(results: Vec<SubmissionResult>) -> Self {
        Self {
            results: Arc::new(results.into_iter().map(|r| TransportResult { submission: r }).collect()),
            call_count: Arc::new(AtomicUsize::new(0)),
            requests: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    fn attempt_count(&self) -> usize {
        self.call_count.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl Transport for SequenceTransport {
    async fn submit_events(
        &self,
        request: SubmissionRequest,
    ) -> Result<SubmissionResult, TransportError> {
        let index = self.call_count.fetch_add(1, Ordering::SeqCst);
        self.requests.lock().unwrap().push(request);
        Ok(self.results[index].submission.clone())
    }
}

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

#[tokio::test]
async fn retry_exhaustion_returns_last_retryable_result() {
    let retry = SubmissionResult::from_response(TransportResponse::new(429, None));
    let inner = SequenceTransport::new(vec![
        retry.clone(),
        retry.clone(),
        retry.clone(),
    ]);

    let policy = retry_policies::policies::ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(1), Duration::from_millis(5))
        .base(2)
        .build_with_max_retries(2);
    let transport = RetryingTransport::new(inner.clone(), policy);

    let result = submit_dummy(&transport).await.unwrap();
    assert_eq!(result.action, exceptionless::transport::SubmissionAction::Retry);
    assert_eq!(inner.attempt_count(), 3);
}

#[tokio::test]
async fn retry_recovery_on_second_attempt() {
    let retry = SubmissionResult::from_response(TransportResponse::new(429, None));
    let success = SubmissionResult::from_response(TransportResponse::new(202, None));
    let inner = SequenceTransport::new(vec![retry, success]);

    let policy = retry_policies::policies::ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(1), Duration::from_millis(5))
        .base(2)
        .build_with_max_retries(2);
    let transport = RetryingTransport::new(inner.clone(), policy);

    let result = submit_dummy(&transport).await.unwrap();
    assert_eq!(
        result.action,
        exceptionless::transport::SubmissionAction::Success
    );
    assert_eq!(inner.attempt_count(), 2);
}

async fn submit_dummy(transport: &impl Transport) -> Result<SubmissionResult, TransportError> {
    let config = ClientConfig::new("test-key").with_server_url("https://example.com");
    let request = SubmissionRequest::from_events(&config, &[]).unwrap();
    transport.submit_events(request).await
}
