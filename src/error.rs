use std::{
    any::{type_name, type_name_of_val},
    error::Error as StdError,
};

use thiserror::Error;

use crate::{
    builder::EventBuilder,
    client::ExceptionlessClient,
    event::Event,
    transport::{SubmissionResult, Transport, TransportError},
    wire::error::{ErrorPayload, StackFrame},
};

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("cannot submit an empty event batch")]
    EmptyBatch,
    #[error(transparent)]
    Transport(#[from] TransportError),
}

#[derive(Debug)]
pub struct ErrorEventBuilder<'a, T: Transport> {
    inner: EventBuilder<'a, T>,
}

impl<'a, T: Transport> ErrorEventBuilder<'a, T> {
    pub(crate) fn new<E>(client: &'a ExceptionlessClient<T>, error: &'a E) -> Self
    where
        E: StdError + 'static + ?Sized,
    {
        let payload = map_error(error);
        let event = Event::error(payload);
        Self {
            inner: EventBuilder::new(client, event),
        }
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner = self.inner.source(source);
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

fn map_error<E>(error: &E) -> ErrorPayload
where
    E: StdError + 'static + ?Sized,
{
    let mut payload = ErrorPayload::new(error.to_string()).with_type(type_name::<E>());

    let debug = format!("{error:?}");
    if !debug.trim().is_empty() {
        payload = payload.with_stack_trace(vec![StackFrame::new(debug)]);
    }

    if let Some(inner) = error.source() {
        payload = payload.with_inner(map_dyn_error(inner));
    }

    payload
}

fn map_dyn_error(error: &(dyn StdError + 'static)) -> ErrorPayload {
    let mut payload = ErrorPayload::new(error.to_string()).with_type(type_name_of_val(error));

    let debug = format!("{error:?}");
    if !debug.trim().is_empty() {
        payload = payload.with_stack_trace(vec![StackFrame::new(debug)]);
    }

    if let Some(inner) = error.source() {
        payload = payload.with_inner(map_dyn_error(inner));
    }

    payload
}
