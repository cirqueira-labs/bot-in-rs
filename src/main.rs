use std::collections::HashMap;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let brave_path = "/usr/bin/brave-browser";
    let mut caps = DesiredCapabilities::chrome();
    let _ = caps.set_binary(&brave_path);
    let _ = caps.set_disable_web_security();
    let driver = WebDriver::new("http://localhost:9515", caps.clone()).await?;
    let mut urls = HashMap::<String, bool>::new();

    //TODO
    //select browser
    //verify webdriver server execution
    //login

    driver
        .get("https://www.linkedin.com/mynetwork/invitation-manager/sent/?page=4")
        .await?;

    while !driver
        .current_url()
        .await?
        .to_string()
        .contains("invitation-manager/sent")
    {}

    sleep(Duration::from_secs(4)).await;
    driver
        .execute("window.scrollTo(0, document.body.scrollHeight);", vec![])
        .await?;

    let elements = driver.find_all(By::Css("a.app-aware-link")).await?;
    for element in elements {
        if let Ok(href) = element.attr("href").await {
            if let Some(link) = href {
                if link.contains("/in/") {
                    urls.insert(link, true);
                }
            }
        }
    }

    for url in urls.keys() {
        let _ = driver.goto(url).await;
        sleep(Duration::from_secs(2)).await;
    }

    driver.quit().await?;

    Ok(())
}
