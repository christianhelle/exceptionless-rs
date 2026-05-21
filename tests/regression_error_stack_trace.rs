#![cfg(not(feature = "opt-out"))]

mod support;

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
};

use exceptionless::ExceptionlessClient;
use support::{CapturingTransport, payload_events, test_config};

#[derive(Debug)]
struct SimpleError;

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "simple error")
    }
}

impl StdError for SimpleError {}

#[tokio::test]
async fn error_stack_trace_contains_real_call_frames() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());

    client.error(&SimpleError).send().await?;

    let requests = transport.requests();
    let events = payload_events(&requests[0]);
    let error_payload = &events[0]["data"]["@error"];

    let frames = error_payload["stack_trace"]
        .as_array()
        .expect("stack_trace should be an array");

    assert!(!frames.is_empty(), "stack trace must not be empty");

    for frame in frames {
        let method = frame["method"]
            .as_str()
            .expect("each frame must have a method");
        assert!(!method.is_empty(), "frame method must not be empty");
        assert!(
            method.contains("::"),
            "frame method should be a qualified Rust path, got: {method}"
        );
    }

    // At least one frame must originate from this test module
    assert!(
        frames.iter().any(|f| f["method"]
            .as_str()
            .is_some_and(|m| m.contains("regression_error_stack_trace"))),
        "expected at least one frame from this test module"
    );

    Ok(())
}

#[tokio::test]
async fn inner_error_does_not_receive_its_own_backtrace() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());

    #[derive(Debug)]
    struct Outer(SimpleError);
    impl Display for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "outer")
        }
    }
    impl StdError for Outer {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            Some(&self.0)
        }
    }

    client.error(&Outer(SimpleError)).send().await?;

    let requests = transport.requests();
    let events = payload_events(&requests[0]);
    let error_payload = &events[0]["data"]["@error"];

    // Outer gets the backtrace
    assert!(
        !error_payload["stack_trace"]
            .as_array()
            .map(|v| v.is_empty())
            .unwrap_or(true),
        "outer error must have a stack trace"
    );

    // Inner error carries no duplicated backtrace of its own
    assert!(
        error_payload["inner"]["stack_trace"]
            .as_array()
            .map(|v| v.is_empty())
            .unwrap_or(true),
        "inner error must not carry its own stack trace"
    );

    Ok(())
}
