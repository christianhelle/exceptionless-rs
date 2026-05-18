# exceptionless

A Rust client for [Exceptionless](https://exceptionless.com) — capture errors, logs, and feature usage events with a clean, async-first API.

## What's Supported

This initial release focuses on core event reporting:

- **Error reporting** — submit exceptions with automatic stack trace capture and inner exception chaining
- **Log events** — send structured logs with severity levels
- **Feature tracking** — record feature usage for analytics
- **Custom metadata** — add tags, user identity, version, and arbitrary data to any event
- **Async/await** — all submission is non-blocking; built on `async-trait` and `reqwest`
- **Bearer authentication** — events submitted with your API key to `collector.exceptionless.io` (or a custom server)

### Not Yet Supported

- Event queuing and batch retry
- Settings/configuration from the Exceptionless server
- Session tracking
- Plugin system
- Log level filtering or structured logging integration (upcoming)

---

## Quick Start

### 1. Add the Dependency

```toml
[dependencies]
exceptionless = "0.1"
```

### 2. Create a Client

The simplest way to get started:

```rust
use exceptionless::ExceptionlessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    
    // Ready to report events
    Ok(())
}
```

### 3. Report an Error

```rust
use exceptionless::ExceptionlessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    
    // Simulate an error
    let result: Result<i32, _> = "not a number".parse();
    
    if let Err(e) = result {
        client.error(&e)
            .tag("parsing")
            .source("user_input")
            .send()
            .await?;
    }
    
    Ok(())
}
```

The client automatically captures the error message and type. Use builder methods to add context:

- `.tag(name)` — label the event (e.g., "auth", "database", "payment")
- `.source(module)` — identify where the error originated
- `.version(v)` — track which version encountered the error
- `.user_identity(id)` — associate the error with a user
- `.data(key, value)` — attach arbitrary metadata

### 4. Send a Log

```rust
use exceptionless::ExceptionlessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    
    client.log("User logged in")
        .level("info")
        .tag("authentication")
        .user_identity("user@example.com")
        .send()
        .await?;
    
    Ok(())
}
```

Available log levels: `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`, `"fatal"`. If not set, the default is `"info"`.

### 5. Track Feature Usage

```rust
use exceptionless::ExceptionlessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ExceptionlessClient::with_api_key("YOUR_API_KEY");
    
    client.feature("export_to_pdf")
        .tag("premium_feature")
        .user_identity("user@example.com")
        .send()
        .await?;
    
    Ok(())
}
```

---

## Configuration

### Custom Server

If you're self-hosting Exceptionless, point the client to your server:

```rust
use exceptionless::ExceptionlessClient;
use exceptionless::config::ClientConfig;
use exceptionless::transport::http::HttpTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("YOUR_API_KEY")
        .with_server_url("https://your-exceptionless-server.com");
    
    let client = ExceptionlessClient::new(config, HttpTransport::default());
    
    client.log("Server configured").send().await?;
    
    Ok(())
}
```

### Disable the Client

For local development or testing, you can disable event submission:

```rust
let config = ClientConfig::new("YOUR_API_KEY")
    .with_enabled(false);
```

When disabled, `send()` will return an error, so tests should either use a test transport or skip submission.

---

## Examples

See the `examples/` directory for runnable patterns:

- `error_basic.rs` — Capture and send a simple error
- `log_structured.rs` — Send logs with custom metadata
- `feature_track.rs` — Record feature usage events
- `config_custom.rs` — Use a custom server URL

Run any example with:

```bash
cargo run --example error_basic
```

---

## Testing

To test your integration without submitting to Exceptionless, provide your own `Transport` implementation. For now, the easiest approach is to use a fake transport in your test harness (details coming in a future update).

---

## Roadmap

See the [GitHub issues](https://github.com/christianhelle/exceptionless.rust/issues) for upcoming features and known limitations.

---

## License

MIT

