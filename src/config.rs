use std::{error::Error as StdError, fmt};

pub const DEFAULT_SERVER_URL: &str = "https://collector.exceptionless.io";
pub const EVENTS_API_PATH: &str = "/api/v2/events";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    api_key: String,
    server_url: String,
    enabled: bool,
}

impl ClientConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            server_url: DEFAULT_SERVER_URL.to_owned(),
            enabled: true,
        }
    }

    pub fn with_server_url(mut self, server_url: impl Into<String>) -> Self {
        self.server_url = server_url.into();
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn server_url(&self) -> &str {
        &self.server_url
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if !self.enabled {
            return Err(ConfigError::Disabled);
        }

        if self.api_key.trim().is_empty() {
            return Err(ConfigError::MissingApiKey);
        }

        reqwest::Url::parse(self.normalized_server_url())
            .map_err(|_| ConfigError::InvalidServerUrl(self.server_url.clone()))?;

        Ok(())
    }

    pub fn events_url(&self) -> Result<String, ConfigError> {
        self.validate()?;

        let base = self.normalized_server_url().trim_end_matches('/');
        Ok(format!("{base}{EVENTS_API_PATH}"))
    }

    fn normalized_server_url(&self) -> &str {
        self.server_url.trim()
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    Disabled,
    MissingApiKey,
    InvalidServerUrl(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disabled => f.write_str("client is disabled"),
            Self::MissingApiKey => f.write_str("api key must not be blank"),
            Self::InvalidServerUrl(url) => write!(f, "invalid server url: {url}"),
        }
    }
}

impl StdError for ConfigError {}
