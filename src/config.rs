//! Advanced configuration primitives for low-level client construction.
//!
//! Most applications should start with [`crate::ExceptionlessClient::with_api_key`].
//! This module is for callers that need to override collector URLs, pre-validate
//! settings, or compose their own transport pipeline.

use std::{error::Error as StdError, fmt};

use url::Url;

/// Default Exceptionless collector used when no custom server URL is configured.
pub const DEFAULT_SERVER_URL: &str = "https://collector.exceptionless.io";
/// Relative API path appended to the configured server URL for event submission.
pub const EVENTS_API_PATH: &str = "/api/v2/events";

/// Advanced client configuration used by the low-level transport pipeline.
///
/// The high-level [`crate::ExceptionlessClient`] constructors usually provide the
/// simplest onboarding path. Reach for [`ClientConfig`] when you need to point the
/// client at a self-hosted Exceptionless server, validate configuration ahead of
/// time, or assemble the client from custom infrastructure.
///
/// # Example
///
/// ```
/// use exceptionless::config::ClientConfig;
///
/// let config = ClientConfig::new("API_KEY")
///     .with_server_url("https://errors.example.com")
///     .with_enabled(true);
///
/// assert!(config.is_valid());
/// assert_eq!(
///     config.events_url().unwrap(),
///     "https://errors.example.com/api/v2/events"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    api_key: String,
    server_url: String,
    enabled: bool,
}

impl ClientConfig {
    /// Creates a configuration with the provided API key and the hosted collector.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            server_url: DEFAULT_SERVER_URL.to_owned(),
            enabled: true,
        }
    }

    /// Overrides the collector base URL.
    ///
    /// This is primarily useful for self-hosted Exceptionless deployments.
    pub fn with_server_url(mut self, server_url: impl Into<String>) -> Self {
        self.server_url = server_url.into();
        self
    }

    /// Enables or disables submission for this configuration.
    ///
    /// When disabled, validation and URL resolution return [`ConfigError::Disabled`].
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Returns the configured API key exactly as stored.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns the configured collector base URL exactly as stored.
    pub fn server_url(&self) -> &str {
        &self.server_url
    }

    /// Returns whether submission is enabled for this configuration.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns `true` when [`Self::validate`] succeeds.
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Validates the current configuration for transport use.
    ///
    /// This checks whether submission is enabled, the API key is non-blank after
    /// trimming, and the server URL parses as an absolute URL.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !self.enabled {
            return Err(ConfigError::Disabled);
        }

        if self.api_key.trim().is_empty() {
            return Err(ConfigError::MissingApiKey);
        }

        Url::parse(self.normalized_server_url())
            .map_err(|_| ConfigError::InvalidServerUrl(self.server_url.clone()))?;

        Ok(())
    }

    /// Resolves the final event submission endpoint.
    ///
    /// This trims surrounding whitespace from the configured server URL, validates
    /// the configuration, and appends [`EVENTS_API_PATH`].
    ///
    /// # Example
    ///
    /// ```
    /// use exceptionless::config::ClientConfig;
    ///
    /// let config = ClientConfig::new("API_KEY")
    ///     .with_server_url(" https://errors.example.com/ ");
    ///
    /// assert_eq!(
    ///     config.events_url().unwrap(),
    ///     "https://errors.example.com/api/v2/events"
    /// );
    /// ```
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
    /// Creates a disabled-in-practice default configuration with a blank API key.
    ///
    /// This is mainly useful for incremental construction in advanced scenarios.
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// Errors returned while validating or resolving low-level client configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// Submission has been explicitly disabled.
    Disabled,
    /// The API key is blank after trimming whitespace.
    MissingApiKey,
    /// The configured server URL could not be parsed as a valid absolute URL.
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
