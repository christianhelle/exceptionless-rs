use std::time::SystemTime;

use async_trait::async_trait;
use retry_policies::{RetryDecision, RetryPolicy};

use super::{SubmissionRequest, SubmissionResult, Transport, TransportError};
use crate::transport::SubmissionAction;

/// Re-export of [`retry_policies::Jitter`] for convenience.
pub use retry_policies::Jitter;
/// Re-export of [`retry_policies::policies::ExponentialBackoff`] for convenience.
pub use retry_policies::policies::ExponentialBackoff;

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
    #[allow(unused_assignments)]
    async fn submit_events(
        &self,
        request: SubmissionRequest,
    ) -> Result<SubmissionResult, TransportError> {
        let start_time = SystemTime::now();
        let mut last_result: Option<Result<SubmissionResult, TransportError>> = None;
        let mut n_past_retries: u32 = 0;

        loop {
            let result = self.inner.submit_events(request.clone()).await;

            let is_retryable = match &result {
                Ok(submission) => submission.action == SubmissionAction::Retry,
                Err(TransportError::Request(_)) => true,
                _ => false,
            };

            if !is_retryable {
                return result;
            }

            last_result = Some(result);

            match self.policy.should_retry(start_time, n_past_retries) {
                RetryDecision::Retry { execute_after } => {
                    let delay = execute_after
                        .duration_since(SystemTime::now())
                        .unwrap_or_default();
                    tokio::time::sleep(delay).await;
                    n_past_retries += 1;
                }
                RetryDecision::DoNotRetry => {
                    return last_result.unwrap();
                }
            }
        }
    }
}
