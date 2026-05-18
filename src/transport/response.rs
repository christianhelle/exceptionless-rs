#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionAction {
    Success,
    Retry,
    SplitAndRetry,
    Discard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportResponse {
    pub status_code: u16,
    pub message: Option<String>,
}

impl TransportResponse {
    pub fn new(status_code: u16, message: Option<String>) -> Self {
        Self {
            status_code,
            message,
        }
    }

    pub fn action(&self) -> SubmissionAction {
        match self.status_code {
            200..=299 => SubmissionAction::Success,
            413 => SubmissionAction::SplitAndRetry,
            408 | 429 | 500..=599 => SubmissionAction::Retry,
            _ => SubmissionAction::Discard,
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self.action(), SubmissionAction::Success)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionResult {
    pub action: SubmissionAction,
    pub response: TransportResponse,
}

impl SubmissionResult {
    pub fn from_response(response: TransportResponse) -> Self {
        let action = response.action();
        Self { action, response }
    }
}
