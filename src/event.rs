//! Ergonomic event wrapper for direct submission scenarios.

use serde_json::Value;

use crate::wire::{error::ErrorPayload, event as wire_event};

/// Public event type used by [`crate::client::ExceptionlessClient::submit`] and
/// [`crate::client::ExceptionlessClient::submit_batch`].
///
/// Most applications should prefer the higher-level builders on
/// [`crate::client::ExceptionlessClient`]. Reach for `Event` when you need to
/// construct an event dynamically or queue it before submission.
///
/// This type is the ergonomic public wrapper. If you need the exact serialized
/// payload model used by transports, see [`crate::wire::event::Event`].
///
/// # Examples
///
/// ```no_run
/// use exceptionless::{client::ExceptionlessClient, event::Event};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
///
///     let event = Event::log("direct event")
///         .with_source("docs")
///         .with_tag("manual")
///         .with_data("attempt", 1.into());
///
///     client.submit(event).await?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    inner: wire_event::Event,
}

impl Event {
    /// Creates an Exceptionless error event from a prepared [`ErrorPayload`].
    ///
    /// This is the low-level alternative to [`crate::client::ExceptionlessClient::capture_error`]
    /// when you already have an Exceptionless-shaped payload.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use exceptionless::{event::Event, wire::error::ErrorPayload};
    ///
    /// let event = Event::error(
    ///     ErrorPayload::new("payment declined")
    ///         .with_type("GatewayError")
    ///         .with_data("gateway", "stripe".into()),
    /// );
    ///
    /// assert_eq!(event.event_type(), "error");
    /// ```
    pub fn error(error: ErrorPayload) -> Self {
        Self {
            inner: wire_event::Event::error(error),
        }
    }

    /// Creates a log event with the provided message.
    pub fn log(message: impl Into<String>) -> Self {
        Self {
            inner: wire_event::Event::log(message),
        }
    }

    /// Creates a feature usage event with the feature name as the source.
    pub fn feature_usage(feature_name: impl Into<String>) -> Self {
        Self {
            inner: wire_event::Event::feature_usage(feature_name),
        }
    }

    /// Returns the Exceptionless event type string.
    pub fn event_type(&self) -> &str {
        &self.inner.event_type
    }

    /// Returns the event source, if one has been set.
    pub fn source(&self) -> Option<&str> {
        self.inner.source.as_deref()
    }

    /// Returns the event message, if one has been set.
    ///
    /// Log events usually carry a message; feature usage events typically do
    /// not.
    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    /// Sets the event source.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.inner = self.inner.with_source(source);
        self
    }

    /// Adds a tag to the event.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.inner = self.inner.with_tag(tag);
        self
    }

    /// Attaches structured data to the event.
    pub fn with_data(mut self, key: impl Into<String>, value: Value) -> Self {
        self.inner = self.inner.with_data(key, value);
        self
    }

    /// Sets the user identity associated with the event.
    pub fn with_user_identity(mut self, identity: impl Into<String>) -> Self {
        self.inner = self.inner.with_user_identity(identity);
        self
    }

    /// Sets the application or deployment version attached to the event.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.inner = self.inner.with_version(version);
        self
    }

    /// Sets log level metadata on the event.
    pub fn with_level(mut self, level: impl AsRef<str>) -> Self {
        self.inner = self.inner.with_level(level);
        self
    }

    #[cfg(not(feature = "opt-out"))]
    pub(crate) fn into_wire(self) -> wire_event::Event {
        self.inner
    }
}
