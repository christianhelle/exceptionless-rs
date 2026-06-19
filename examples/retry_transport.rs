use std::time::Duration;

use exceptionless::ExceptionlessClient;
use exceptionless::transport::http::HttpTransport;
use exceptionless::transport::retry::{ExponentialBackoff, Jitter, RetryingTransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a custom retry policy:
    //   - min delay: 100ms
    //   - max delay: 5s
    //   - full jitter (randomize each delay)
    //   - base 2 (double each retry)
    //   - up to 3 retries (4 total attempts)
    let policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(100), Duration::from_secs(5))
        .jitter(Jitter::Full)
        .base(2)
        .build_with_max_retries(3);

    // Wrap the default HTTP transport with retry
    let transport = RetryingTransport::new(HttpTransport::default(), policy);

    let config = exceptionless::config::ClientConfig::new("YOUR_API_KEY_HERE");
    let client = ExceptionlessClient::new(config, transport);

    // A network blip will be retried automatically up to 3 times
    client
        .log("Event with automatic retry")
        .tag("retry-example")
        .send()
        .await?;

    println!("Event sent (possibly after retries)");

    // Alternatively, use the one-liner convenience constructor:
    let client = ExceptionlessClient::with_api_key_and_retry("YOUR_API_KEY_HERE");

    client
        .log("Built-in retry client")
        .tag("retry-example")
        .send()
        .await?;

    println!("Event sent via built-in retry client");

    Ok(())
}
