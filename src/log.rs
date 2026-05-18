use crate::{
    builder::EventBuilder,
    error::ClientError,
    transport::{SubmissionResult, Transport},
};

#[derive(Debug)]
pub struct LogEventBuilder<'a, T: Transport> {
    inner: EventBuilder<'a, T>,
}

impl<'a, T: Transport> LogEventBuilder<'a, T> {
    pub(crate) fn new(inner: EventBuilder<'a, T>) -> Self {
        Self { inner }
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner = self.inner.source(source);
        self
    }

    pub fn level(mut self, level: impl AsRef<str>) -> Self {
        self.inner = self.inner.map_event(|event| event.with_level(level));
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.inner = self.inner.tag(tag);
        self
    }

    pub fn data(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.inner = self.inner.data(key, value);
        self
    }

    pub fn user_identity(mut self, identity: impl Into<String>) -> Self {
        self.inner = self.inner.user_identity(identity);
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.inner = self.inner.version(version);
        self
    }

    pub async fn send(self) -> Result<SubmissionResult, ClientError> {
        self.inner.send().await
    }
}
