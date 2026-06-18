use std::time::Duration;

use exceptionless::ExceptionlessClient;
use exceptionless::config::ClientConfig;
use exceptionless::transport::http::HttpTransport;
use exceptionless::transport::retry::{ExponentialBackoff, Jitter, RetryingTransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the client to use a custom server
    let config = ClientConfig::new("YOUR_API_KEY_HERE")
        .with_server_url("https://your-exceptionless-server.com");

    let client = ExceptionlessClient::new(config, HttpTransport::default());

    // Send a log to the custom server
    client
        .log("Connected to custom Exceptionless server")
        .tag("setup")
        .send()
        .await?;

    println!("Log sent to custom server");

    // --- With retry support ---

    let config = ClientConfig::new("YOUR_API_KEY_HERE")
        .with_server_url("https://your-exceptionless-server.com");

    let policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(200), Duration::from_secs(10))
        .jitter(Jitter::Full)
        .base(2)
        .build_with_max_retries(2);

    let transport = RetryingTransport::new(HttpTransport::default(), policy);
    let client = ExceptionlessClient::new(config, transport);

    client
        .log("Connected with retry support")
        .tag("setup")
        .tag("retry")
        .send()
        .await?;

    println!("Log sent with retry support");

    Ok(())
}
