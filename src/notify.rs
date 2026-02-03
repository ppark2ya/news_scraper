use std::env;
use std::error::Error;
use reqwest::blocking::Client;
use serde_json::json;
// use serde::Serialize;

// #[derive(Serialize)]
// struct TelegramMessage {
//     chat_id: String,
//     text: String,
//     parse_mode: String,
// }

// pub fn send_telegram(message: &str) -> Result<(), Box<dyn Error>> {
//     let token = env::var("TELEGRAM_TOKEN").unwrap_or_default();
//     let chat_id = env::var("TELEGRAM_CHAT_ID").unwrap_or_default();

//     if token.is_empty() || chat_id.is_empty() {
//         println!("Warning: TELEGRAM_TOKEN or TELEGRAM_CHAT_ID is not set. Skipping notification.");
//         println!("--- Mock Notification ---\n{}", message);
//         return Ok(());
//     }

//     let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
//     let payload = TelegramMessage {
//         chat_id,
//         text: message.to_string(),
//         parse_mode: "HTML".to_string(),
//     };

//     let client = Client::new();
//     let resp = client.post(&url).json(&payload).send()?;

//     if !resp.status().is_success() {
//         println!("Telegram Request failed: {}", resp.status());
//     }

//     Ok(())
// }

pub fn send_to_slack(message: &str) -> Result<(), Box<dyn Error>> {
    let webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    let payload = json!({ "text": message });
    // let payload = json!({
    //     "blocks": [
    //         {
    //             "type": "header",
    //             "text": {
    //                 "type": "plain_text",
    //                 "text": "📰 2026-02-03 모닝 브리핑"
    //             }
    //         },
    //         {
    //             "type": "section",
    //             "text": {
    //                 "type": "mrkdwn",
    //                 "text": "*[속보] 코스피 3000 돌파*\n오늘 아침 개장과 동시에..."
    //             },
    //             "accessory": {
    //                 "type": "button",
    //                 "text": {
    //                     "type": "plain_text",
    //                     "text": "원문 보기"
    //                 },
    //                 "url": "https://hankyung.com/..."
    //             }
    //         },
    //         {
    //             "type": "divider"
    //         }
    //     ]
    // });
    let response = Client::new()
        .post(&webhook_url)
        .json(&payload)
        .send()?;

    if response.status().is_success() {
        println!("Success!");
    } else {
        println!("Failed!");
    }

    Ok(())
}
