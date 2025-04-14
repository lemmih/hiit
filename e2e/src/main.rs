use anyhow::{ensure, Context, Result};
use reqwest::StatusCode;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

mod settings;
use settings::test_settings_presets;

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

    let ret = tests(&driver).await;

    // Clean up
    driver.quit().await.context("Failed to quit WebDriver session")?;

    match ret {
        Ok(_) => {
            println!("E2E test passed successfully!");
            Ok(())
        }
        Err(e) => {
            println!("E2E test failed!");
            Err(e)
        }
    }
}

async fn tests(driver: &WebDriver) -> Result<()> {
    check_title(driver, "HIIT").await?;

    // Verify the header is present
    {
        let header = driver.find(By::Tag("h1")).await.context("Failed to find header")?;
        let header_text = header.text().await.context("Failed to get header text")?;
        ensure!(
            header_text.contains("HIIT Workout App"),
            "Header '{}' does not contain 'HIIT Workout App'",
            header_text
        );
    }

    // Check built-in routines
    {
        // Find cards with class cursor-pointer and make sure that there are 3 or more.
        let cards = driver
            .find_all(By::Css(".cursor-pointer"))
            .await
            .context("Failed to find cards")?;
        ensure!(
            cards.len() >= 2,
            "Expected at least 2 cards with class 'cursor-pointer', but found {}",
            cards.len()
        );
    }

    // Verify that routines are clickable
    {
        // Find the first routine and click it
        let first_routine = driver
            .find(By::Css(".cursor-pointer"))
            .await
            .context("Failed to find first routine")?;
        first_routine.click().await.context("Failed to click first routine")?;
        // Check that there is a button with the text "Start Routine"
        let start_routine_button = driver
            .find(By::XPath("//button[contains(text(), 'Start Routine')]"))
            .await
            .context("Failed to find start routine button")?;
        ensure!(start_routine_button
            .is_displayed()
            .await
            .context("Start routine button is not displayed")?,);
        // Navigate back to the start page
        driver.back().await.context("Failed to navigate back")?;
    }

    // Test settings
    test_settings_presets(driver).await?;
    // test_settings_persistence(driver).await?;

    Ok(())
}

async fn check_title(driver: &WebDriver, expected_title: &str) -> Result<()> {
    // Navigate to the website
    driver
        .goto("http://localhost:8787")
        .await
        .context("Failed to navigate to website")?;

    // Get the page title
    let title = driver.title().await.context("Failed to get page title")?;

    // Check if title contains expected text
    ensure!(
        title.contains(expected_title),
        "Page title '{}' does not contain '{}'",
        title,
        expected_title
    );
    Ok(())
}
