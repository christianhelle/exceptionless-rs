use core::num::ParseIntError;
use exceptionless::ExceptionlessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ExceptionlessClient::with_api_key("YOUR_API_KEY_HERE");

    // Simulate a parsing error
    let result = parse();

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

fn parse() -> Result<i32, ParseIntError> {
    parse_string_as_integer("not a number")
}

fn parse_string_as_integer(text: &str) -> Result<i32, ParseIntError> {
    text.parse()
}
