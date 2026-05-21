//! Response classification helpers for low-level transport implementations.
//!
//! These types encode the current retry guidance used by the built-in transport and
//! any custom transport that wants to stay wire-compatible with the crate.

/// Suggested handling strategy for a completed submission attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionAction {
    /// The batch was accepted and does not need another attempt.
    Success,
    /// The same batch may succeed if retried later.
    Retry,
    /// The batch should be split into smaller chunks before retrying.
    SplitAndRetry,
    /// The batch should be dropped because retrying is not expected to help.
    Discard,
}

/// Minimal response details needed to classify a submission outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportResponse {
    /// HTTP status code returned by the collector.
    pub status_code: u16,
    /// Optional server or transport message associated with the response.
    pub message: Option<String>,
}

impl TransportResponse {
    /// Creates a low-level response wrapper from a status code and optional message.
    pub fn new(status_code: u16, message: Option<String>) -> Self {
        Self {
            status_code,
            message,
        }
    }

    /// Maps the status code to the crate's retry guidance.
    ///
    /// Status `413` is treated specially as [`SubmissionAction::SplitAndRetry`].
    pub fn action(&self) -> SubmissionAction {
        match self.status_code {
            200..=299 => SubmissionAction::Success,
            413 => SubmissionAction::SplitAndRetry,
            408 | 429 | 500..=599 => SubmissionAction::Retry,
            _ => SubmissionAction::Discard,
        }
    }

    /// Returns `true` when [`Self::action`] resolves to [`SubmissionAction::Success`].
    pub fn is_success(&self) -> bool {
        matches!(self.action(), SubmissionAction::Success)
    }
}

/// Combined response details and the derived submission action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionResult {
    /// Retry guidance derived from [`Self::response`].
    pub action: SubmissionAction,
    /// Low-level response details from the transport backend.
    pub response: TransportResponse,
}

impl SubmissionResult {
    /// Classifies a raw response and stores both the action and original details.
    pub fn from_response(response: TransportResponse) -> Self {
        let action = response.action();
        Self { action, response }
    }
}
