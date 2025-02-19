use std::error::Error;
use std::fs;
use tokio;

// Feels hacky, maybe improve this
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = reqwest::get("http://localhost:8080/private/openapi.json").await?;
    let body = response.text().await?;

    fs::write("shared/openapi.json", body)?;

    println!("OpenAPI spec saved successfully.");
    Ok(())
}
