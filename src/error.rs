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

    let frames = capture_backtrace();
    if !frames.is_empty() {
        payload = payload.with_stack_trace(frames);
    }

    if let Some(inner) = error.source() {
        payload = payload.with_inner(map_dyn_error(inner));
    }

    payload
}

fn map_dyn_error(error: &(dyn StdError + 'static)) -> ErrorPayload {
    let mut payload = ErrorPayload::new(error.to_string()).with_type(type_name_of_val(error));

    if let Some(inner) = error.source() {
        payload = payload.with_inner(map_dyn_error(inner));
    }

    payload
}

fn capture_backtrace() -> Vec<StackFrame> {
    let bt = backtrace::Backtrace::new();
    let mut frames = Vec::new();

    for frame in bt.frames() {
        for symbol in frame.symbols() {
            let name = symbol.name().map(|n| format!("{n:#}")).unwrap_or_default();

            if name.is_empty() || is_noise_frame(&name) {
                continue;
            }

            let mut sf = StackFrame::new(&name);
            if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                sf = sf.with_file(file.to_string_lossy().into_owned(), line);
                if let Some(col) = symbol.colno() {
                    sf = sf.with_column(col);
                }
            }
            frames.push(sf);
        }
    }

    frames
}

fn is_noise_frame(name: &str) -> bool {
    // Non-Rust symbols (Windows/libc bootstrap, C functions) have no `::`
    if !name.contains("::") {
        return true;
    }
    const NOISE_PREFIXES: &[&str] = &[
        "exceptionless::",
        "backtrace::",
        "std::rt::",
        "std::sys::",
        "std::panicking::",
        "std::panic::",
        "std::thread::",
        "core::ops::function::",
        "core::hint::",
        "core::future::",
        "core::panic::",
        "tokio::runtime::",
        "tokio::task::",
        "test::",
        "__rust_",
    ];
    NOISE_PREFIXES.iter().any(|p| name.starts_with(p))
}
