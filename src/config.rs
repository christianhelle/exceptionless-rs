use thiserror::Error;

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

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConfigError {
    #[error("client is disabled")]
    Disabled,
    #[error("api key must not be blank")]
    MissingApiKey,
    #[error("invalid server url: {0}")]
    InvalidServerUrl(String),
}
