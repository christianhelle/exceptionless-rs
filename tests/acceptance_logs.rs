mod support;

use std::error::Error as StdError;

use exceptionless::ExceptionlessClient;

use support::{payload_events, test_config, CapturingTransport};

#[tokio::test]
async fn log_entrypoint_sets_log_contract_and_trims_level() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());

    client
        .log("worker started")
        .source("jobs")
        .level(" Warn ")
        .send()
        .await?;

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);

    let events = payload_events(&requests[0]);
    let event = &events[0];

    assert_eq!(event["type"], "log");
    assert_eq!(event["message"], "worker started");
    assert_eq!(event["source"], "jobs");
    assert_eq!(event["data"]["@level"], "Warn");

    Ok(())
}

#[tokio::test]
async fn log_entrypoint_omits_blank_level() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config(), transport.clone());

    client.log("worker started").level("   ").send().await?;

    let requests = transport.requests();
    let events = payload_events(&requests[0]);
    let event = events[0].as_object().expect("event should be an object");
    let data = event.get("data").and_then(|value| value.as_object());

    assert!(data.is_none_or(|value| !value.contains_key("@level")));

    Ok(())
}
