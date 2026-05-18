mod support;

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
    // Stack trace must contain real call frames (qualified Rust paths, not error Debug reprs)
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
    assert_eq!(error_payload["inner"]["message"], "inner boom");

    Ok(())
}
