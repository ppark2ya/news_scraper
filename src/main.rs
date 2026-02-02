mod scrapers;
mod notify;

use scrapers::{Scraper, nyt::NytScraper, guardian::GuardianScraper, korea::KoreaScraper};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Daily News Scraper...");

    let mut all_news = Vec::new();

    // 1. New York Times
    println!("Fetching NYT...");
    let nyt = NytScraper;
    match nyt.fetch() {
        Ok(mut items) => all_news.append(&mut items),
        Err(e) => println!("Error fetching NYT: {}", e),
    }

    // 2. Guardian
    println!("Fetching Guardian...");
    let guardian = GuardianScraper;
    match guardian.fetch() {
        Ok(mut items) => all_news.append(&mut items),
        Err(e) => println!("Error fetching Guardian: {}", e),
    }

    // 3. Korean News
    println!("Fetching Korean News...");
    let korea = KoreaScraper;
    match korea.fetch() {
        Ok(mut items) => all_news.append(&mut items),
        Err(e) => println!("Error fetching Korean News: {}", e),
    }

    // 4. Format Message
    let mut message = String::from("<b>Daily News Digest</b>\n\n");
    if all_news.is_empty() {
        message.push_str("No news found or potential errors.");
    } else {
        for item in all_news.iter() {
            // Telegram has a message length limit (4096 chars), careful not to exceed.
            // Limiting to ~20 items max if needed, but we gather about 5+5+12=22 items.
            // It should be fine.
            let entry = format!(
                "<b>[{}] {}</b>\n<i>{}</i>\n<a href=\"{}\">Read More</a>\n\n",
                item.source, item.title, item.published_date, item.url
            );
            if message.len() + entry.len() > 4000 {
                message.push_str("\n... (truncated)");
                break;
            }
            message.push_str(&entry);
        }
    }

    // 5. Send Notification
    println!("Sending Notification...");
    notify::send_telegram(&message)?;

    println!("Done.");
    Ok(())
}