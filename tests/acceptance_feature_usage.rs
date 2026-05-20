#![cfg(not(feature = "opt-out"))]

mod support;

use std::error::Error as StdError;

use exceptionless::ExceptionlessClient;

use support::{CapturingTransport, payload_events, test_config};

#[tokio::test]
async fn feature_entrypoint_maps_name_to_usage_source() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());

    client.feature("search").send().await?;

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);

    let events = payload_events(&requests[0]);
    let event = events[0].as_object().expect("event should be an object");

    assert_eq!(event["type"], "usage");
    assert_eq!(event["source"], "search");
    assert!(!event.contains_key("message"));

    Ok(())
}
