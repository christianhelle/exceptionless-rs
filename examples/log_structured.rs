use exceptionless::ExceptionlessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ExceptionlessClient::with_api_key("YOUR_API_KEY_HERE");

    // Send an info-level log with metadata
    client
        .log("User login attempt")
        .level("info")
        .tag("authentication")
        .tag("login")
        .user_identity("user@example.com")
        .data("ip_address", "192.168.1.100")
        .data("browser", "Firefox")
        .send()
        .await?;

    println!("Log event sent");

    // Send a warning-level log
    client
        .log("High memory usage detected")
        .level("warn")
        .tag("performance")
        .data("memory_mb", 2048)
        .send()
        .await?;

    println!("Warning log sent");

    Ok(())
}
