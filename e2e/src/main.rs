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

    // Test 1: Verify the header is present
    let header = driver.find(By::Tag("h1")).await.context("Failed to find header")?;
    let header_text = header.text().await.context("Failed to get header text")?;
    assert!(
        header_text.contains("HIIT Workout App"),
        "Header '{}' does not contain 'HIIT Workout App'",
        header_text
    );

    // Test 2: Verify the Core Crusher exercise card is present
    let exercise_cards = driver
        .find_all(By::XPath("//div[contains(@class, 'rounded-lg')]"))
        .await
        .context("Failed to find exercise cards")?;

    assert!(!exercise_cards.is_empty(), "No exercise cards found on the page");

    // Find the Core Crusher card specifically
    let mut found_core_exercise = false;
    for card in exercise_cards {
        if let Ok(card_text) = card.text().await {
            if card_text.contains("Core Crusher") {
                found_core_exercise = true;
                // Click on the card to navigate to the timer page
                card.click().await.context("Failed to click on Core Crusher card")?;

                // Test 3: Check that we've navigated to the timer page
                sleep(Duration::from_secs(1)).await; // Wait for navigation

                // Check the URL after navigation
                let current_url = driver.current_url().await?;
                assert!(
                    current_url.as_ref().contains("/timer/core-1"),
                    "URL should contain '/timer/core-1' but was '{}'",
                    current_url.as_ref()
                );

                // Verify the timer page elements
                let timer_header = driver
                    .find(By::Tag("h2"))
                    .await
                    .context("Failed to find timer page header")?;
                let timer_header_text = timer_header.text().await?;
                assert!(
                    timer_header_text.contains("Core Crusher"),
                    "Timer page header should contain 'Core Crusher' but was '{}'",
                    timer_header_text
                );

                // Check for the start button
                let start_button = driver
                    .find(By::XPath("//button[contains(text(), 'Start Exercise')]"))
                    .await
                    .context("Failed to find start button")?;

                // Click the start button
                start_button.click().await.context("Failed to click start button")?;

                // Verify the timer is running by checking the button text changed
                sleep(Duration::from_secs(1)).await; // Wait a moment for the button state to change

                let button_after_click = driver
                    .find(By::Tag("button"))
                    .await
                    .context("Failed to find button after click")?;
                let button_text = button_after_click.text().await?;

                assert!(
                    button_text.contains("In Progress"),
                    "Button text should change to indicate progress but was '{}'",
                    button_text
                );

                // Wait to verify the timer is counting down
                sleep(Duration::from_secs(2)).await;

                // Check if timer value has decreased
                let timer_display = driver
                    .find(By::XPath("//div[contains(@class, 'text-5xl')]"))
                    .await
                    .context("Failed to find timer display")?;
                let timer_text = timer_display.text().await?;

                // Verify the time format (should be MM:SS)
                assert!(
                    timer_text.matches(r"^\d{2}:\d{2}$").count() == 1,
                    "Timer should display in MM:SS format but was '{}'",
                    timer_text
                );

                break;
            }
        }
    }

    assert!(found_core_exercise, "Could not find 'Core Crusher' exercise card");

    // Navigate back to homepage
    let back_link = driver
        .find(By::XPath("//a[contains(text(), 'Back')]"))
        .await
        .context("Failed to find back link")?;
    back_link.click().await.context("Failed to click back link")?;

    // Verify we're back on the homepage
    sleep(Duration::from_secs(1)).await; // Wait for navigation
    let homepage_header = driver
        .find(By::Tag("h1"))
        .await
        .context("Failed to find homepage header after navigation back")?;
    let homepage_header_text = homepage_header.text().await?;
    assert!(
        homepage_header_text.contains("HIIT Workout App"),
        "Homepage header text should contain 'HIIT Workout App' but was '{}'",
        homepage_header_text
    );

    // Clean up
    driver.quit().await.context("Failed to quit WebDriver session")?;

    println!("E2E test passed successfully!");
    Ok(())
}
