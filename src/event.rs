use serde_json::Value;

use crate::wire::{error::ErrorPayload, event as wire_event};

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    inner: wire_event::Event,
}

impl Event {
    pub fn error(error: ErrorPayload) -> Self {
        Self {
            inner: wire_event::Event::error(error),
        }
    }

    pub fn log(message: impl Into<String>) -> Self {
        Self {
            inner: wire_event::Event::log(message),
        }
    }

    pub fn feature_usage(feature_name: impl Into<String>) -> Self {
        Self {
            inner: wire_event::Event::feature_usage(feature_name),
        }
    }

    pub fn event_type(&self) -> &str {
        &self.inner.event_type
    }

    pub fn source(&self) -> Option<&str> {
        self.inner.source.as_deref()
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.inner = self.inner.with_source(source);
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.inner = self.inner.with_tag(tag);
        self
    }

    pub fn with_data(mut self, key: impl Into<String>, value: Value) -> Self {
        self.inner = self.inner.with_data(key, value);
        self
    }

    pub fn with_user_identity(mut self, identity: impl Into<String>) -> Self {
        self.inner = self.inner.with_user_identity(identity);
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.inner = self.inner.with_version(version);
        self
    }

    pub fn with_level(mut self, level: impl AsRef<str>) -> Self {
        self.inner = self.inner.with_level(level);
        self
    }

    #[cfg(not(feature = "opt-out"))]
    pub(crate) fn into_wire(self) -> wire_event::Event {
        self.inner
    }
}
