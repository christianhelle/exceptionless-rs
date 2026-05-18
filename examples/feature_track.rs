use exceptionless::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_api_key("YOUR_API_KEY_HERE");

    // Track a feature usage
    client
        .feature("export_to_pdf")
        .tag("premium_feature")
        .user_identity("user@example.com")
        .data("document_type", "report")
        .data("page_count", 15)
        .send()
        .await?;

    println!("Feature usage tracked: export_to_pdf");

    // Track another feature
    client
        .feature("advanced_search")
        .user_identity("user@example.com")
        .data("query_length", 42)
        .data("results_count", 156)
        .send()
        .await?;

    println!("Feature usage tracked: advanced_search");

    Ok(())
}
