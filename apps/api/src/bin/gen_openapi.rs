use colored::*;
use std::fs;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start the server in the background
    let mut server = Command::new("cargo").args(["run"]).spawn()?;

    // Wait for the server to be ready
    let mut retries = 0;
    while retries < 30 {
        if reqwest::get("http://localhost:8080/api/private/openapi.json")
            .await
            .is_ok()
        {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        retries += 1;
    }

    if retries >= 30 {
        eprintln!("Server failed to start within 30 seconds");
        server.kill()?;
        return Ok(());
    }

    // Fetch the OpenAPI spec
    let response = reqwest::get("http://localhost:8080/api/private/openapi.json").await?;
    let spec = response.text().await?;

    // Write the spec to packages/api/openapi.json
    fs::write("../../packages/api/openapi.json", spec)?;
    println!(
        "{}",
        "🔧 openapi spec generated -> ../../packages/api/openapi.json".green()
    );

    // Kill the server
    server.kill()?;

    Ok(())
}
