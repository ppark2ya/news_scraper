use std::env;
use std::error::Error;
use reqwest::blocking::Client;

pub fn send_to_slack(payload: &serde_json::Value) -> Result<(), Box<dyn Error>> {
    let webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    let response = Client::new()
        .post(&webhook_url)
        .json(payload)
        .send()?;

    if response.status().is_success() {
        println!("Success!");
    } else {
        println!("Failed! Status: {}", response.status());
        println!("Body: {}", response.text().unwrap_or_default());
    }

    Ok(())
}
