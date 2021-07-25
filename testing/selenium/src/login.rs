use thirtyfour::prelude::*;
use tokio;
use tokio::time::{sleep, Duration};

pub async fn login_with_redirect(driver: &WebDriver) -> WebDriverResult<()> {
    // Login logout with redirect
    let login_with_redirect_button = driver.find_element(By::Id("login-with-redirect")).await?;
    login_with_redirect_button.click().await?;

    let form_element = driver.find_element(By::Css("form[method='POST']")).await?;
    let username_form = form_element.find_element(By::Id("username")).await?;
    username_form.send_keys("test@test.com").await?;
    let password_form = form_element.find_element(By::Id("password")).await?;
    password_form.send_keys("Qwerty12345!").await?;

    let submit_button = form_element.find_element(By::Css("button[type='submit']")).await?;
    submit_button.click().await?;

    let refresh_button = driver.find_element(By::Id("refresh")).await?;
    refresh_button.click().await?;

    // Assert data here

    let logout_button = driver.find_element(By::Id("logout")).await?;
    logout_button.click().await?;

    handle_consent(driver).await
}

pub async fn login_with_popup(driver: &WebDriver) -> WebDriverResult<()> {
    // Login logout with popup
    let login_with_redirect_button = driver.find_element(By::Id("login-with-popup")).await?;
    login_with_redirect_button.click().await?;

    sleep(Duration::from_millis(200)).await;
    let window_handles = driver.window_handles().await?;
    driver.switch_to().window(&window_handles[1]).await?;

    let form_element = driver.find_element(By::Css("form[method='POST']")).await?;
    let username_form = form_element.find_element(By::Id("username")).await?;
    username_form.send_keys("test@test.com").await?;
    let password_form = form_element.find_element(By::Id("password")).await?;
    password_form.send_keys("Qwerty12345!").await?;

    let submit_button = form_element.find_element(By::Css("button[type='submit']")).await?;
    submit_button.click();

    driver.switch_to().window(&window_handles[0]).await?;

    let refresh_button = driver.find_element(By::Id("refresh")).await?;
    refresh_button.click().await?;

    let logout_button = driver.find_element(By::Id("logout")).await?;
    logout_button.click().await?;

    handle_consent(driver).await
}

async fn handle_consent(driver: &WebDriver) -> WebDriverResult<()> {
    match driver.find_element(By::Css("form[method='POST']")).await {
        Ok(consent_element) => {
            let submit_concent_button = consent_element.find_element(By::Css("button[value='accept']")).await?;
            submit_concent_button.click().await?;
        }
        Err(_) => (),
    }
    Ok(())
}
