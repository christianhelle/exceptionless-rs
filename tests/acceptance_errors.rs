mod support;

use core::num::ParseIntError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
};

use exceptionless::ExceptionlessClient;
use serde_json::json;

use support::{payload_events, test_config, CapturingTransport};

#[derive(Debug)]
struct InnerError;

impl Display for InnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "inner boom")
    }
}

impl StdError for InnerError {}

#[derive(Debug)]
struct OuterError {
    source: InnerError,
}

impl Display for OuterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "outer boom")
    }
}

impl StdError for OuterError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

#[tokio::test]
async fn error_entrypoint_shapes_payload_and_preserves_context() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());
    let error = OuterError { source: InnerError };

    client
        .error(&error)
        .source("checkout")
        .tag("ops")
        .tag("ops")
        .tag("   ")
        .data("tenant", json!("acme"))
        .user_identity("user-42")
        .version("1.2.3")
        .send()
        .await?;

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);

    let events = payload_events(&requests[0]);
    assert_eq!(events.len(), 1);

    let event = &events[0];
    assert_eq!(event["type"], "error");
    assert_eq!(event["source"], "checkout");
    assert_eq!(event["tags"], json!(["ops"]));
    assert_eq!(event["data"]["tenant"], "acme");
    assert_eq!(event["data"]["@user"], "user-42");
    assert_eq!(event["data"]["@version"], "1.2.3");
    assert!(event["date"]
        .as_str()
        .is_some_and(|value| !value.is_empty()));

    let error_payload = &event["data"]["@error"];
    assert_eq!(error_payload["message"], "outer boom");
    assert!(error_payload["type"]
        .as_str()
        .is_some_and(|value| value.ends_with("OuterError")));

    // Stack trace must be non-empty, contain qualified Rust paths, and carry file+line info
    let frames = error_payload["stack_trace"]
        .as_array()
        .expect("stack_trace should be an array");
    assert!(!frames.is_empty(), "stack_trace should not be empty");
    assert!(
        frames[0]["method"]
            .as_str()
            .is_some_and(|m| m.contains("::")),
        "first frame method should be a qualified Rust path, got: {}",
        frames[0]["method"]
    );
    assert!(
        frames
            .iter()
            .any(|f| f["file_name"].as_str().is_some_and(|v| v.ends_with(".rs"))),
        "no frame had a .rs file_name"
    );
    assert!(
        frames.iter().any(|f| f["line_number"].is_number()),
        "no frame had a line_number"
    );

    assert_eq!(error_payload["inner"]["message"], "inner boom");

    Ok(())
}

#[tokio::test]
async fn stack_trace_from_stdlib_error_has_real_frames() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());

    let err: ParseIntError = "not a number".parse::<i32>().unwrap_err();
    client.error(&err).send().await?;

    let requests = transport.requests();
    let events = payload_events(&requests[0]);
    let error_payload = &events[0]["data"]["@error"];
    let frames = error_payload["stack_trace"]
        .as_array()
        .expect("stack_trace should be an array");

    // A single debug-dump entry is not a stack trace
    assert!(
        frames.len() > 1,
        "expected multiple stack frames, got {}",
        frames.len()
    );

    // At least one frame must have a .rs file name
    assert!(
        frames
            .iter()
            .any(|f| f["file_name"].as_str().is_some_and(|v| v.ends_with(".rs"))),
        "no frame had a .rs file_name — stack trace looks like a debug dump, not real frames"
    );

    // At least one frame must have a line number
    assert!(
        frames.iter().any(|f| f["line_number"].is_number()),
        "no frame had a line_number — stack trace is missing source location"
    );

    // No frame's method should look like a raw Debug repr "SomeType { field: value }"
    for frame in frames {
        let method = frame["method"].as_str().unwrap_or("");
        assert!(
            !method.contains('{') || method.contains("::"),
            "frame method looks like a raw Debug repr, not a function name: {method:?}"
        );
    }

    Ok(())
}

#[tokio::test]
async fn stack_trace_frames_include_error_site() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());

    let err: ParseIntError = "bad".parse::<i32>().unwrap_err();
    client.error(&err).send().await?;

    let requests = transport.requests();
    let events = payload_events(&requests[0]);
    let error_payload = &events[0]["data"]["@error"];
    let frames = error_payload["stack_trace"]
        .as_array()
        .expect("stack_trace should be an array");

    // At least one frame must reference this test file — not only SDK internals
    assert!(
        frames.iter().any(|f| {
            f["file_name"]
                .as_str()
                .is_some_and(|v| v.contains("acceptance_errors"))
        }),
        "no frame referenced user code — backtrace may be capturing only SDK frames"
    );

    Ok(())
}
