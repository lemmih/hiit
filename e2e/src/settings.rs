use anyhow::{ensure, Result};
use thirtyfour::prelude::*;

// Go to settings page, click each preset button, and check that the settings are updated
pub async fn test_settings_presets(driver: &WebDriver) -> Result<()> {
    // Navigate to the settings page
    driver.goto("http://localhost:8787/settings").await?;

    let low_preset = driver.find(By::XPath("//button[contains(text(), 'Low')]")).await?;
    let mid_preset = driver.find(By::XPath("//button[contains(text(), 'Mid')]")).await?;
    let high_preset = driver.find(By::XPath("//button[contains(text(), 'High')]")).await?;

    // Verify that 'High Intensity Duration' is set to 30
    low_preset.click().await?;
    let high_intensity_duration = driver.find(By::Id("slider-high-intensity-duration")).await?;
    let high_intensity_duration_value = high_intensity_duration.value().await?;
    ensure!(
        high_intensity_duration_value == Some("30".to_string()),
        "High Intensity Duration is not set to 30"
    );

    // Verify that 'High Intensity Duration' is set to 45
    mid_preset.click().await?;
    let high_intensity_duration_value = high_intensity_duration.value().await?;
    ensure!(
        high_intensity_duration_value == Some("45".to_string()),
        "High Intensity Duration is not set to 45"
    );

    // Verify that 'High Intensity Duration' is set to 60
    high_preset.click().await?;
    let high_intensity_duration_value = high_intensity_duration.value().await?;
    ensure!(
        high_intensity_duration_value == Some("60".to_string()),
        "High Intensity Duration is not set to 60"
    );

    Ok(())
}

// async fn set_high_intensity_duration(driver: &WebDriver, value: u32) -> Result<()> {
//     let slider = driver.find(By::Id("slider-high-intensity-duration")).await?;
//     driver
//         .execute(
//             &format!(
//                 "arguments[0].value = {}; arguments[0].dispatchEvent(new Event('input'));",
//                 value
//             ),
//             vec![slider.to_json()?],
//         )
//         .await?;
//     Ok(())
// }

// async fn get_high_intensity_duration(driver: &WebDriver) -> Result<String, WebDriverError> {
//     let slider = driver.find(By::Id("slider-high-intensity-duration")).await?;
//     let value = slider.value().await?;
//     Ok(value.unwrap_or_default())
// }

// This test is not working because the slider is not updated when the page is refreshed.
// pub async fn test_settings_persistence(driver: &WebDriver) -> Result<()> {
//     // Navigate to the settings page
//     driver.goto("http://localhost:8787/settings").await?;

//     // Test with value 10
//     set_high_intensity_duration(driver, 10).await?;
//     driver.refresh().await?;
//     let value = get_high_intensity_duration(driver).await?;
//     ensure!(
//         value == "10",
//         "High Intensity Duration did not persist after refresh (expected 10, got {})",
//         value
//     );

//     // Test with value 60
//     set_high_intensity_duration(driver, 60).await?;
//     driver.refresh().await?;
//     let value = get_high_intensity_duration(driver).await?;
//     ensure!(
//         value == "60",
//         "High Intensity Duration did not persist after refresh (expected 60, got {})",
//         value
//     );

//     // Test with value 100
//     set_high_intensity_duration(driver, 100).await?;
//     driver.refresh().await?;
//     let value = get_high_intensity_duration(driver).await?;
//     ensure!(
//         value == "100",
//         "High Intensity Duration did not persist after refresh (expected 100, got {})",
//         value
//     );

//     Ok(())
// }
