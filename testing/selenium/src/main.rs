mod common;
mod login;

use thirtyfour::prelude::*;
use tokio;
use tokio::time::{sleep, Duration};

use login::{login_with_redirect, login_with_popup};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;

    driver.get("http://localhost:8000").await?;
    // Wait for page to load
    sleep(Duration::from_millis(200)).await;

    login_with_redirect(&driver).await;
    login_with_popup(&driver).await;

    sleep(Duration::from_millis(200)).await;
    // driver.quit().await?;
    Ok(())
}
