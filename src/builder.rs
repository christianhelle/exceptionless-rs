//! Shared event builder used by the typed public builders.

use serde_json::Value;

use crate::{
    client::ExceptionlessClient,
    error::ClientError,
    event::Event,
    transport::{SubmissionResult, Transport},
};

/// Shared fluent builder for event metadata.
///
/// Most users interact with this behavior through the more specific builders
/// returned by [`crate::client::ExceptionlessClient::capture_error`],
/// [`crate::client::ExceptionlessClient::log`], and
/// [`crate::client::ExceptionlessClient::feature`]. This type holds the common
/// metadata operations that all of those builders expose.
#[derive(Debug)]
pub struct EventBuilder<'a, T: Transport> {
    client: &'a ExceptionlessClient<T>,
    event: Event,
}

impl<'a, T: Transport> EventBuilder<'a, T> {
    pub(crate) fn new(client: &'a ExceptionlessClient<T>, event: Event) -> Self {
        Self { client, event }
    }

    /// Sets the logical event source.
    ///
    /// For log and error events this usually maps to the subsystem or component
    /// that emitted the event.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.event = self.event.with_source(source);
        self
    }

    /// Adds a tag to the event.
    ///
    /// Empty tags are ignored and duplicate tags are not added twice.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.event = self.event.with_tag(tag);
        self
    }

    /// Attaches structured data to the event.
    ///
    /// Values are converted into [`serde_json::Value`], which makes this method
    /// work well with strings, numbers, booleans, and pre-built JSON values.
    pub fn data(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.event = self.event.with_data(key, value.into());
        self
    }

    /// Sets the user identity associated with the event.
    pub fn user_identity(mut self, identity: impl Into<String>) -> Self {
        self.event = self.event.with_user_identity(identity);
        self
    }

    /// Sets the application or deployment version attached to the event.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.event = self.event.with_version(version);
        self
    }

    /// Returns the current event snapshot without sending it.
    ///
    /// This is useful in tests and when you want to inspect the final event
    /// shape before calling [`Self::send`].
    pub fn event(&self) -> &Event {
        &self.event
    }

    pub(crate) fn map_event(mut self, map: impl FnOnce(Event) -> Event) -> Self {
        self.event = map(self.event);
        self
    }

    /// Submits the built event through the owning client.
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
    ///         .log("event builder send")
    ///         .tag("docs")
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(self) -> Result<SubmissionResult, ClientError> {
        self.client.submit(self.event).await
    }
}
