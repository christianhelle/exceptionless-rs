//! Feature usage event builder.

use crate::{
    builder::EventBuilder,
    error::ClientError,
    transport::{SubmissionResult, Transport},
};

/// Fluent builder for feature usage events.
///
/// Obtain this from [`crate::client::ExceptionlessClient::feature`]. Feature
/// usage events are intentionally lightweight: the feature name becomes the
/// event source and any extra dimensions can be attached through tags or
/// structured data.
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
///         .feature("bulk_export")
///         .tag("beta")
///         .data("format", "csv")
///         .data("row_count", 512)
///         .user_identity("user@example.com")
///         .version("1.8.0")
///         .send()
///         .await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct FeatureUsageBuilder<'a, T: Transport> {
    inner: EventBuilder<'a, T>,
}

impl<'a, T: Transport> FeatureUsageBuilder<'a, T> {
    pub(crate) fn new(inner: EventBuilder<'a, T>) -> Self {
        Self { inner }
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

    /// Submits the feature usage event.
    pub async fn send(self) -> Result<SubmissionResult, ClientError> {
        self.inner.send().await
    }
}
