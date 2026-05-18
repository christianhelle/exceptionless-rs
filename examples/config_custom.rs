use exceptionless::config::ClientConfig;
use exceptionless::transport::http::HttpTransport;
use exceptionless::ExceptionlessClient;

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

    Ok(())
}
