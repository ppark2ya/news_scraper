mod scrapers;
mod notify;

use scrapers::{Scraper, nyt::NytScraper, guardian::GuardianScraper, korea::KoreaScraper, cnbc::CnbcScraper, techcrunch::TechCrunchScraper};
use std::{error::Error};
use serde_json::json;

fn get_source_emoji(source: &str) -> &'static str {
    match source {
        "CNBC Business" => "💹",
        "CNBC Economy" => "📊",
        "TechCrunch" => "🚀",
        "Guardian" => "🇬🇧",
        "NYT" => "🗽",
        "MK" | "Yonhap" => "🇰🇷",
        _ => "📰",
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    println!("Starting Daily News Scraper...");

    let mut all_news = Vec::new();

    // 1. New York Times
    // println!("Fetching NYT...");
    // let nyt = NytScraper;
    // match nyt.fetch() {
    //     Ok(mut items) => all_news.append(&mut items),
    //     Err(e) => println!("Error fetching NYT: {}", e),
    // }

    // println!("NYT: {:#?}", &all_news);

    // 3. Korean News
    // println!("Fetching Korean News...");
    // let korea = KoreaScraper;
    // match korea.fetch() {
    //     Ok(mut items) => all_news.append(&mut items),
    //     Err(e) => println!("Error fetching Korean News: {}", e),
    // }

    // println!("Korea: {:#?}", &all_news);

    // 1. CNBC
    println!("Fetching CNBC...");
    let cnbc = CnbcScraper;
    match cnbc.fetch() {
        Ok(mut items) => all_news.append(&mut items),
        Err(e) => println!("Error fetching CNBC: {}", e),
    }

    // 2. TechCrunch
    println!("Fetching TechCrunch...");
    let techcrunch = TechCrunchScraper;
    match techcrunch.fetch() {
        Ok(mut items) => all_news.append(&mut items),
        Err(e) => println!("Error fetching TechCrunch: {}", e),
    }

    // 3. Guardian
    println!("Fetching Guardian...");
    let guardian = GuardianScraper;
    match guardian.fetch() {
        Ok(mut items) => all_news.append(&mut items),
        Err(e) => println!("Error fetching Guardian: {}", e),
    }

    println!("Guardian: {:#?}", &all_news);

    // 4. Format Message
    let mut blocks = Vec::new();

    // Header
    blocks.push(json!({
        "type": "header",
        "text": {
            "type": "plain_text",
            "text": "Daily News Digest 📰",
            "emoji": true
        }
    }));
    blocks.push(json!({ "type": "divider" }));

    if all_news.is_empty() {
        blocks.push(json!({
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "No news found or potential errors."
            }
        }));
    } else {
        for item in all_news.iter() {
            // Limit to avoid hitting Slack's block limit (50 blocks). 
            // Header + Divider = 2 blocks. Each item = 2 blocks (section + divider).
            // So roughly 20-24 items max.
            if blocks.len() > 40 {
               blocks.push(json!({
                   "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": "... (truncated)"
                    }
               }));
               break;
            }

            blocks.push(json!({
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": format!("{} *[{}]* {}\n_{}_", get_source_emoji(&item.source), item.source, item.title, item.published_date)
                },
                "accessory": {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "Read More",
                        "emoji": true
                    },
                    "url": item.url
                }
            }));
            blocks.push(json!({ "type": "divider" }));
        }
    }

    let payload = json!({
        "blocks": blocks,
        "text": "Daily News Digest" // Fallback text for notifications
    });

    // 5. Send Notification
    notify::send_to_slack(&payload).expect("Failed to send to Slack");

    println!("Done.");
    Ok(())
}