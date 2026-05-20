use std::{error::Error as StdError, fmt};

#[cfg(feature = "http")]
pub mod http;
pub mod response;

use async_trait::async_trait;
use serde_json::to_string;

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

#[derive(Debug)]
pub enum TransportError {
    InvalidConfiguration(ConfigError),
    Serialization(serde_json::Error),
    Request(String),
    ResponseBody(String),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(error) => write!(f, "{error}"),
            Self::Serialization(error) => {
                write!(f, "failed to serialize event payload: {error}")
            }
            Self::Request(message) => write!(f, "request failed: {message}"),
            Self::ResponseBody(message) => write!(f, "failed to read response body: {message}"),
        }
    }
}

impl StdError for TransportError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidConfiguration(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::Request(_) | Self::ResponseBody(_) => None,
        }
    }
}

impl From<ConfigError> for TransportError {
    fn from(error: ConfigError) -> Self {
        Self::InvalidConfiguration(error)
    }
}

impl From<serde_json::Error> for TransportError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}
