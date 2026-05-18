use std::error::Error as StdError;

use crate::{
    builder::EventBuilder,
    config::ClientConfig,
    error::{ClientError, ErrorEventBuilder},
    event::Event,
    feature::FeatureUsageBuilder,
    log::LogEventBuilder,
    transport::{http::HttpTransport, SubmissionRequest, SubmissionResult, Transport},
};

#[derive(Debug, Clone)]
pub struct ExceptionlessClient<T: Transport = HttpTransport> {
    config: ClientConfig,
    transport: T,
}

impl ExceptionlessClient<HttpTransport> {
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self::new(ClientConfig::new(api_key), HttpTransport::default())
    }
}

impl<T: Transport> ExceptionlessClient<T> {
    pub fn new(config: ClientConfig, transport: T) -> Self {
        Self { config, transport }
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn transport(&self) -> &T {
        &self.transport
    }

    pub fn error<'a, E>(&'a self, error: &'a E) -> ErrorEventBuilder<'a, T>
    where
        E: StdError + 'static + ?Sized,
    {
        ErrorEventBuilder::new(self, error)
    }

    pub fn log<'a>(&'a self, message: impl Into<String>) -> LogEventBuilder<'a, T> {
        LogEventBuilder::new(EventBuilder::new(self, Event::log(message)))
    }

    pub fn feature<'a>(&'a self, feature_name: impl Into<String>) -> FeatureUsageBuilder<'a, T> {
        FeatureUsageBuilder::new(EventBuilder::new(self, Event::feature_usage(feature_name)))
    }

    pub async fn submit(&self, event: Event) -> Result<SubmissionResult, ClientError> {
        self.submit_batch([event]).await
    }

    pub async fn submit_batch<I>(&self, events: I) -> Result<SubmissionResult, ClientError>
    where
        I: IntoIterator<Item = Event>,
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
