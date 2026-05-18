pub mod http;
pub mod response;

use async_trait::async_trait;
use serde_json::to_string;
use thiserror::Error;

use crate::{
    config::{ClientConfig, ConfigError},
    wire::event::Event,
};

pub use response::{SubmissionAction, SubmissionResult, TransportResponse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionRequest {
    pub endpoint: String,
    pub authorization: String,
    pub payload: String,
}

impl SubmissionRequest {
    pub fn from_events(config: &ClientConfig, events: &[Event]) -> Result<Self, TransportError> {
        let endpoint = config.events_url()?;
        let payload = to_string(events)?;

        Ok(Self {
            endpoint,
            authorization: format!("Bearer {}", config.api_key().trim()),
            payload,
        })
    }
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn submit_events(
        &self,
        request: SubmissionRequest,
    ) -> Result<SubmissionResult, TransportError>;
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    InvalidConfiguration(#[from] ConfigError),
    #[error("failed to serialize event payload: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("request failed: {0}")]
    Request(String),
    #[error("failed to read response body: {0}")]
    ResponseBody(String),
}
