use anyhow::{Context, Result};
use reqwest::StatusCode;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

async fn wait_for_service(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    for _ in 0..60 {
        match client.get(url).send().await {
            Ok(response) if response.status() == StatusCode::OK => return Ok(()),
            _ => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    anyhow::bail!("Service did not become ready within 60 seconds")
}

async fn wait_for_webdriver(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    for _ in 0..60 {
        match client.get(url).send().await {
            Ok(_) => return Ok(()),
            _ => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    anyhow::bail!("WebDriver did not become ready within 60 seconds")
}

#[tokio::main]
async fn main() -> Result<()> {
    // Wait for the web service to be ready
    wait_for_service("http://localhost:8787")
        .await
        .context("Failed waiting for web service")?;

    // Wait for ChromeDriver to be ready
    wait_for_webdriver("http://localhost:4444")
        .await
        .context("Failed waiting for ChromeDriver")?;

    // Connect to WebDriver instance
    let mut caps = DesiredCapabilities::firefox();
    // let mut caps = DesiredCapabilities::safari();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .context("Failed to connect to WebDriver")?;

    // Navigate to the website
    driver
        .goto("http://localhost:8787")
        .await
        .context("Failed to navigate to website")?;

    // Get the page title
    let title = driver.title().await.context("Failed to get page title")?;

    // Check if title contains expected text
    assert!(title.contains("HIIT"), "Page title '{}' does not contain 'HIIT'", title);

    // Clean up
    driver.quit().await.context("Failed to quit WebDriver session")?;

    println!("E2E test passed successfully!");
    Ok(())
}
