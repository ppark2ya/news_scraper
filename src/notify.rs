use std::env;
use std::error::Error;
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: String,
    text: String,
    parse_mode: String,
}

pub fn send_telegram(message: &str) -> Result<(), Box<dyn Error>> {
    let token = env::var("TELEGRAM_TOKEN").unwrap_or_default();
    let chat_id = env::var("TELEGRAM_CHAT_ID").unwrap_or_default();

    if token.is_empty() || chat_id.is_empty() {
        println!("Warning: TELEGRAM_TOKEN or TELEGRAM_CHAT_ID is not set. Skipping notification.");
        println!("--- Mock Notification ---\n{}", message);
        return Ok(());
    }

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let payload = TelegramMessage {
        chat_id,
        text: message.to_string(),
        parse_mode: "HTML".to_string(),
    };

    let client = Client::new();
    let resp = client.post(&url).json(&payload).send()?;

    if !resp.status().is_success() {
        println!("Telegram Request failed: {}", resp.status());
    }

    Ok(())
}
