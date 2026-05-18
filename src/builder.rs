use serde_json::Value;

use crate::{
    client::Client,
    error::ClientError,
    event::Event,
    transport::{SubmissionResult, Transport},
};

#[derive(Debug)]
pub struct EventBuilder<'a, T: Transport> {
    client: &'a Client<T>,
    event: Event,
}

impl<'a, T: Transport> EventBuilder<'a, T> {
    pub(crate) fn new(client: &'a Client<T>, event: Event) -> Self {
        Self { client, event }
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.event = self.event.with_source(source);
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.event = self.event.with_tag(tag);
        self
    }

    pub fn data(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.event = self.event.with_data(key, value.into());
        self
    }

    pub fn user_identity(mut self, identity: impl Into<String>) -> Self {
        self.event = self.event.with_user_identity(identity);
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.event = self.event.with_version(version);
        self
    }

    pub fn event(&self) -> &Event {
        &self.event
    }

    pub(crate) fn map_event(mut self, map: impl FnOnce(Event) -> Event) -> Self {
        self.event = map(self.event);
        self
    }

    pub async fn send(self) -> Result<SubmissionResult, ClientError> {
        self.client.submit(self.event).await
    }
}
