mod support;

use std::error::Error as StdError;

use exceptionless::{
    ClientError, ExceptionlessClient,
    config::{ClientConfig, ConfigError},
    event::Event,
    transport::TransportError,
};

use support::{CapturingTransport, payload_events, test_config};

#[tokio::test]
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
