use async_trait::async_trait;
use retry_policies::policies::ExponentialBackoff;

use super::{SubmissionRequest, SubmissionResult, Transport, TransportError};

/// Transport decorator that retries failed submissions with exponential jittered backoff.
///
/// Wraps any [`Transport`] implementation and automatically retries when the inner
/// transport returns a retryable result or a network-level error.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
/// use exceptionless::transport::http::HttpTransport;
/// use exceptionless::transport::retry::RetryingTransport;
///
/// let inner = HttpTransport::default();
/// let policy = retry_policies::policies::ExponentialBackoff::builder()
///     .retry_bounds(Duration::from_millis(200), Duration::from_secs(10))
///     .jitter(retry_policies::Jitter::Full)
///     .base(2)
///     .build_with_max_retries(3);
///
/// let transport = RetryingTransport::new(inner, policy);
/// ```
#[derive(Debug, Clone)]
pub struct RetryingTransport<T: Transport> {
    inner: T,
    policy: ExponentialBackoff,
}

impl<T: Transport> RetryingTransport<T> {
    /// Wraps an existing transport with a retry policy.
    ///
    /// `inner` — the base transport to decorate.
    /// `policy` — the retry policy (e.g. [`ExponentialBackoff`]) that governs
    ///            when and how long to wait between attempts.
    pub fn new(inner: T, policy: ExponentialBackoff) -> Self {
        Self { inner, policy }
    }
}

#[async_trait]
impl<T: Transport> Transport for RetryingTransport<T> {
    async fn submit_events(
        &self,
        request: SubmissionRequest,
    ) -> Result<SubmissionResult, TransportError> {
        self.inner.submit_events(request).await
    }
}
