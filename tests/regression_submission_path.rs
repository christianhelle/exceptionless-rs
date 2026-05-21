mod support;

use std::{error::Error as StdError, io};

#[cfg(not(feature = "opt-out"))]
use exceptionless::event::Event;
#[cfg(feature = "opt-out")]
use exceptionless::transport::SubmissionAction;
use exceptionless::{
    ClientError, ExceptionlessClient,
    config::{ClientConfig, ConfigError},
    transport::TransportError,
};

use support::CapturingTransport;
#[cfg(not(feature = "opt-out"))]
use support::{payload_events, test_config};

#[tokio::test]
#[cfg(not(feature = "opt-out"))]
async fn submit_batch_uses_configured_endpoint_and_bearer_token() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(
        test_config().with_server_url("https://alt.example.com/root/"),
        transport.clone(),
    );

    client
        .submit_batch([Event::log("first"), Event::feature_usage("search")])
        .await?;

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);

    let request = &requests[0];
    assert_eq!(
        request.endpoint,
        "https://alt.example.com/root/api/v2/events"
    );
    assert_eq!(request.authorization, "Bearer test-api-key");

    let events = payload_events(request);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["type"], "log");
    assert_eq!(events[1]["type"], "usage");

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "opt-out"))]
async fn submit_batch_trims_server_url_before_building_endpoint() -> Result<(), Box<dyn StdError>> {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(
        test_config().with_server_url("  https://alt.example.com/root/  "),
        transport.clone(),
    );

    client.submit_batch([Event::log("first")]).await?;

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests[0].endpoint,
        "https://alt.example.com/root/api/v2/events"
    );

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "opt-out"))]
async fn disabled_config_fails_before_transport_submission() {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(test_config().with_enabled(false), transport.clone());

    let error = client
        .log("blocked")
        .send()
        .await
        .expect_err("config should fail");

    assert!(matches!(
        error,
        ClientError::Transport(TransportError::InvalidConfiguration(ConfigError::Disabled))
    ));
    assert!(transport.requests().is_empty());
}

#[tokio::test]
#[cfg(not(feature = "opt-out"))]
async fn blank_api_key_fails_before_transport_submission() {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(
        ClientConfig::new("   ").with_server_url("https://example.com"),
        transport.clone(),
    );

    let error = client
        .feature("blocked")
        .send()
        .await
        .expect_err("missing api key should fail");

    assert!(matches!(
        error,
        ClientError::Transport(TransportError::InvalidConfiguration(
            ConfigError::MissingApiKey
        ))
    ));
    assert!(transport.requests().is_empty());
}

#[test]
fn exceptionless_client_with_api_key_constructor_remains_available() {
    let client = ExceptionlessClient::with_api_key("test-api-key");

    assert_eq!(client.config().api_key(), "test-api-key");
}

#[tokio::test]
#[cfg(feature = "opt-out")]
async fn opt_out_feature_returns_success_without_transport_or_config_validation() {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(
        ClientConfig::new("   ")
            .with_server_url("https:// bad")
            .with_enabled(false),
        transport.clone(),
    );

    let result = client
        .feature("blocked")
        .send()
        .await
        .expect("opt-out should short-circuit successfully");

    assert!(result.response.is_success());
    assert_eq!(result.response.status_code, 202);
    assert!(transport.requests().is_empty());
}

#[tokio::test]
#[cfg(feature = "opt-out")]
async fn opt_out_submit_batch_empty_batch_returns_synthetic_accepted_without_transport() {
    let transport = CapturingTransport::success();
    let client = ExceptionlessClient::new(
        ClientConfig::new("   ")
            .with_server_url("https:// bad")
            .with_enabled(false),
        transport.clone(),
    );

    let result = client
        .submit_batch(Vec::new())
        .await
        .expect("opt-out should accept empty batches without validation");

    assert_eq!(result.action, SubmissionAction::Success);
    assert!(result.response.is_success());
    assert_eq!(result.response.status_code, 202);
    assert!(result.response.message.is_none());
    assert!(transport.requests().is_empty());
}

#[test]
fn config_error_display_messages_are_stable() {
    let disabled = ConfigError::Disabled;
    assert_eq!(disabled.to_string(), "client is disabled");
    assert!(disabled.source().is_none());

    let missing_api_key = ConfigError::MissingApiKey;
    assert_eq!(missing_api_key.to_string(), "api key must not be blank");
    assert!(missing_api_key.source().is_none());

    let invalid_server_url = ConfigError::InvalidServerUrl("https:// bad".into());
    assert_eq!(
        invalid_server_url.to_string(),
        "invalid server url: https:// bad"
    );
    assert!(invalid_server_url.source().is_none());
}

#[test]
fn transport_error_display_messages_and_sources_are_stable() {
    let invalid_config = TransportError::from(ConfigError::MissingApiKey);
    assert_eq!(invalid_config.to_string(), "api key must not be blank");
    let config_source = invalid_config.source().expect("config source should exist");
    assert_eq!(config_source.to_string(), "api key must not be blank");
    assert!(config_source.downcast_ref::<ConfigError>().is_some());
    assert!(config_source.source().is_none());

    let serialization =
        TransportError::from(serde_json::Error::io(io::Error::other("encode boom")));
    assert_eq!(
        serialization.to_string(),
        "failed to serialize event payload: encode boom"
    );
    let serialization_source = serialization
        .source()
        .expect("serialization source should exist");
    assert_eq!(serialization_source.to_string(), "encode boom");
    assert!(
        serialization_source
            .downcast_ref::<serde_json::Error>()
            .is_some()
    );

    let request = TransportError::Request("timeout".into());
    assert_eq!(request.to_string(), "request failed: timeout");
    assert!(request.source().is_none());

    let response_body = TransportError::ResponseBody("connection closed".into());
    assert_eq!(
        response_body.to_string(),
        "failed to read response body: connection closed"
    );
    assert!(response_body.source().is_none());
}

#[test]
fn client_error_display_messages_and_sources_are_stable() {
    let empty_batch = ClientError::EmptyBatch;
    assert_eq!(
        empty_batch.to_string(),
        "cannot submit an empty event batch"
    );
    assert!(empty_batch.source().is_none());

    let wrapped = ClientError::from(TransportError::from(ConfigError::MissingApiKey));
    assert_eq!(wrapped.to_string(), "api key must not be blank");

    let transport_source = wrapped.source().expect("transport source should exist");
    assert_eq!(transport_source.to_string(), "api key must not be blank");
    assert!(transport_source.downcast_ref::<TransportError>().is_some());

    let config_source = transport_source
        .source()
        .expect("config source should exist");
    assert_eq!(config_source.to_string(), "api key must not be blank");
    assert!(config_source.downcast_ref::<ConfigError>().is_some());
    assert!(config_source.source().is_none());
}
