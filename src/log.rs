//! Log event builder.

use crate::{
    builder::EventBuilder,
    error::ClientError,
    transport::{SubmissionResult, Transport},
};

/// Fluent builder for log events.
///
/// Obtain this from [`crate::client::ExceptionlessClient::log`]. Log events are
/// a convenient way to send operational breadcrumbs and structured diagnostics
/// through the same pipeline as errors.
///
/// # Examples
///
/// ```no_run
/// use exceptionless::ExceptionlessClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
///
///     client
///         .log("invoice queued")
///         .source("worker.billing")
///         .level("info")
///         .tag("queue")
///         .data("invoice_id", "inv_123")
///         .user_identity("user@example.com")
///         .version("1.0.0")
///         .send()
///         .await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct LogEventBuilder<'a, T: Transport> {
    inner: EventBuilder<'a, T>,
}

impl<'a, T: Transport> LogEventBuilder<'a, T> {
    pub(crate) fn new(inner: EventBuilder<'a, T>) -> Self {
        Self { inner }
    }

    /// Sets the logical event source.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner = self.inner.source(source);
        self
    }

    /// Sets the log level metadata.
    ///
    /// The SDK forwards the value as-is after trimming whitespace, so values
    /// like `"trace"`, `"info"`, `"warn"`, and `"error"` work naturally.
    pub fn level(mut self, level: impl AsRef<str>) -> Self {
        self.inner = self.inner.map_event(|event| event.with_level(level));
        self
    }

    /// Adds a tag to the event.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.inner = self.inner.tag(tag);
        self
    }

    /// Attaches structured data to the event.
    pub fn data(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.inner = self.inner.data(key, value);
        self
    }

    /// Sets the user identity associated with the event.
    pub fn user_identity(mut self, identity: impl Into<String>) -> Self {
        self.inner = self.inner.user_identity(identity);
        self
    }

    /// Sets the application or deployment version attached to the event.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.inner = self.inner.version(version);
        self
    }

    /// Submits the log event.
    pub async fn send(self) -> Result<SubmissionResult, ClientError> {
        self.inner.send().await
    }
}
