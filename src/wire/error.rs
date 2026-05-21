//! Advanced error payload types embedded into Exceptionless events.
//!
//! These models are useful when you need exact control over the serialized
//! `@error` shape or want to bypass the higher-level error builder pipeline.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Wire-format error payload stored under [`super::event::DATA_KEY_ERROR`].
///
/// This type is primarily an advanced escape hatch for callers who need to shape
/// nested errors or stack frames directly before submission.
///
/// # Example
///
/// ```
/// use exceptionless::wire::error::{ErrorPayload, StackFrame};
/// use serde_json::json;
///
/// let payload = ErrorPayload::new("database unavailable")
///     .with_type("DbError")
///     .with_stack_trace(vec![
///         StackFrame::new("repository::load").with_file("src/repository.rs", 42),
///     ])
///     .with_data("tenant", json!("demo"));
///
/// assert_eq!(payload.message.as_deref(), Some("database unavailable"));
/// assert_eq!(payload.error_type.as_deref(), Some("DbError"));
/// ```
///
/// If you copy this example into your application, add `serde_json` as a direct
/// dependency in your own crate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub struct ErrorPayload {
    /// Optional human-readable error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional application-specific error type name.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    /// Ordered stack frames associated with the error.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stack_trace: Vec<StackFrame>,
    /// Nested inner error payload, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner: Option<Box<ErrorPayload>>,
    /// Arbitrary structured data attached to the error.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub data: Map<String, Value>,
}

impl ErrorPayload {
    /// Creates an error payload with a message and otherwise empty fields.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
            ..Self::default()
        }
    }

    /// Sets the application-specific error type.
    pub fn with_type(mut self, error_type: impl Into<String>) -> Self {
        self.error_type = Some(error_type.into());
        self
    }

    /// Replaces the stack trace with the provided frames.
    pub fn with_stack_trace(mut self, stack_trace: Vec<StackFrame>) -> Self {
        self.stack_trace = stack_trace;
        self
    }

    /// Stores a nested inner error payload.
    pub fn with_inner(mut self, inner: ErrorPayload) -> Self {
        self.inner = Some(Box::new(inner));
        self
    }

    /// Inserts structured error data under the provided key.
    pub fn with_data(mut self, key: impl Into<String>, value: Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }
}

/// Single frame within an [`ErrorPayload`] stack trace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct StackFrame {
    /// Fully-qualified method or function name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Source file associated with the frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// One-based source line number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<u32>,
    /// One-based source column number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,
}

impl StackFrame {
    /// Creates a stack frame with a method name.
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            method: Some(method.into()),
            ..Self::default()
        }
    }

    /// Sets the source file and line number for the frame.
    pub fn with_file(mut self, file_name: impl Into<String>, line_number: u32) -> Self {
        self.file_name = Some(file_name.into());
        self.line_number = Some(line_number);
        self
    }

    /// Sets the source column number for the frame.
    pub fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }
}
