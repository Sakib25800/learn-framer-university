use colored::*;
use std::error::Error;
use std::fs;
use tokio;

// Hacky, need to improve this
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = reqwest::get("http://localhost:8080/private/openapi.json")
        .await
        .expect("Server is not running");
    let body = response.text().await?;

    fs::write("shared/api/openapi.json", body)?;

    println!(
        "{}",
        "🚀 openapi spec generated -> ./shared/api/openapi.json".green()
    );

    Ok(())
}
