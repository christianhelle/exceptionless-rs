//! Public client facade for the ergonomic SDK entry points.

use std::error::Error as StdError;

use crate::{
    builder::EventBuilder,
    config::ClientConfig,
    error::{ClientError, ErrorEventBuilder},
    event::Event,
    feature::FeatureUsageBuilder,
    log::LogEventBuilder,
    transport::{SubmissionResult, Transport},
};

#[cfg(not(feature = "opt-out"))]
use crate::transport::SubmissionRequest;
#[cfg(feature = "opt-out")]
use crate::transport::TransportResponse;
use crate::transport::http::HttpTransport;

/// High-level Exceptionless client.
///
/// This is the main entry point for the crate. Most applications create one
/// client and reuse it for all error, log, and feature usage submissions.
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
///         .log("worker started")
///         .source("jobs")
///         .level("info")
///         .tag("startup")
///         .send()
///         .await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ExceptionlessClient<T: Transport = HttpTransport> {
    config: ClientConfig,
    transport: T,
}

impl ExceptionlessClient<HttpTransport> {
    /// Creates a client that uses the built-in [`HttpTransport`] and the
    /// default hosted Exceptionless collector.
    ///
    /// This is the most direct way to get started when you only need to supply
    /// an API key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use exceptionless::ExceptionlessClient;
    ///
    /// let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    /// assert_eq!(client.config().api_key(), "YOUR_API_KEY");
    /// ```
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self::new(ClientConfig::new(api_key), HttpTransport::default())
    }
}

#[cfg(feature = "opt-out")]
fn opt_out_submission_result() -> SubmissionResult {
    SubmissionResult::from_response(TransportResponse::new(202, None))
}

impl<T: Transport> ExceptionlessClient<T> {
    /// Creates a client from an explicit [`ClientConfig`] and transport.
    ///
    /// Use this when you need a custom server URL, a test transport, or your
    /// own transport implementation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use exceptionless::{
    ///     client::ExceptionlessClient,
    ///     config::ClientConfig,
    ///     transport::http::HttpTransport,
    /// };
    ///
    /// let config = ClientConfig::new("YOUR_API_KEY")
    ///     .with_server_url("https://your-exceptionless-server.example");
    ///
    /// let client = ExceptionlessClient::new(config, HttpTransport::default());
    /// assert!(client.config().is_valid());
    /// ```
    pub fn new(config: ClientConfig, transport: T) -> Self {
        Self { config, transport }
    }

    /// Returns the configuration used for submissions.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Returns the transport responsible for dispatching submission requests.
    ///
    /// This is mainly useful for tests, diagnostics, or advanced transport
    /// customization.
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Starts building an error event from an existing Rust error.
    ///
    /// The builder captures the error message, its type name, the chained inner
    /// error sources, and a filtered stack trace before submission.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use exceptionless::ExceptionlessClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    ///     let parse_error = "abc".parse::<u32>().unwrap_err();
    ///
    ///     client
    ///         .capture_error(&parse_error)
    ///         .source("user_input")
    ///         .tag("validation")
    ///         .data("raw_value", "abc")
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn capture_error<'a, E>(&'a self, error: &'a E) -> ErrorEventBuilder<'a, T>
    where
        E: StdError + 'static + ?Sized,
    {
        ErrorEventBuilder::new(self, error)
    }

    /// Starts building a log event.
    ///
    /// Log events are a good fit for operational breadcrumbs and structured
    /// diagnostics that should flow through the same Exceptionless pipeline as
    /// errors.
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
    ///         .log("cache warmed")
    ///         .source("background_worker")
    ///         .level("info")
    ///         .data("item_count", 128)
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn log<'a>(&'a self, message: impl Into<String>) -> LogEventBuilder<'a, T> {
        LogEventBuilder::new(EventBuilder::new(self, Event::log(message)))
    }

    /// Starts building a feature usage event.
    ///
    /// Exceptionless models feature usage by putting the feature name in the
    /// event source field and any extra dimensions in event data or tags.
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
    ///         .feature("export_pdf")
    ///         .user_identity("user@example.com")
    ///         .version("1.4.0")
    ///         .data("template", "invoice")
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn feature<'a>(&'a self, feature_name: impl Into<String>) -> FeatureUsageBuilder<'a, T> {
        FeatureUsageBuilder::new(EventBuilder::new(self, Event::feature_usage(feature_name)))
    }

    /// Submits a single event.
    ///
    /// This is the low-level escape hatch when you already have an [`Event`]
    /// value and do not need one of the typed builders returned by
    /// [`Self::capture_error`], [`Self::log`], or [`Self::feature`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use exceptionless::{client::ExceptionlessClient, event::Event};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    ///     let event = Event::log("submitted directly").with_tag("manual");
    ///
    ///     client.submit(event).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn submit(&self, event: Event) -> Result<SubmissionResult, ClientError> {
        self.submit_batch([event]).await
    }

    /// Submits a batch of events in a single request.
    ///
    /// Returns [`ClientError::EmptyBatch`] when the iterator produces no
    /// events. When the `opt-out` Cargo feature is enabled, this method returns
    /// a synthetic accepted response without invoking the transport.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use exceptionless::{client::ExceptionlessClient, event::Event};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    ///     let events = [
    ///         Event::log("batch item 1").with_tag("batch"),
    ///         Event::feature_usage("import_csv").with_data("rows", 42.into()),
    ///     ];
    ///
    ///     client.submit_batch(events).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn submit_batch<I>(&self, events: I) -> Result<SubmissionResult, ClientError>
    where
        I: IntoIterator<Item = Event>,
    {
        #[cfg(feature = "opt-out")]
        {
            drop(events);
            Ok(opt_out_submission_result())
        }

        #[cfg(not(feature = "opt-out"))]
        {
            let events: Vec<_> = events.into_iter().map(Event::into_wire).collect();
            if events.is_empty() {
                return Err(ClientError::EmptyBatch);
            }

            let request = SubmissionRequest::from_events(&self.config, &events)?;
            self.transport
                .submit_events(request)
                .await
                .map_err(ClientError::from)
        }
    }
}
