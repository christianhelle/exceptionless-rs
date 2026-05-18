use exceptionless::ExceptionlessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ExceptionlessClient::with_api_key("YOUR_API_KEY_HERE");

    // Simulate a parsing error
    let result: Result<i32, _> = "not a number".parse();

    match result {
        Err(e) => {
            // Send the error to Exceptionless
            client
                .error(&e)
                .tag("parsing")
                .source("user_input")
                .send()
                .await?;
            println!("Error reported to Exceptionless");
        }
        Ok(num) => {
            println!("Parsed number: {}", num);
        }
    }

    Ok(())
}
