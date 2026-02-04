use super::{NewsItem, Scraper};
use rss::Channel;
use std::error::Error;
use reqwest::blocking::Client;
use std::io::BufReader;

pub struct TechCrunchScraper;

impl Scraper for TechCrunchScraper {
    fn fetch(&self) -> Result<Vec<NewsItem>, Box<dyn Error>> {
        let url = "https://techcrunch.com/feed/";

        let client = Client::builder()
            .user_agent("Mozilla/5.0 (compatible; NewsBot/1.0)")
            .build()?;

        let mut all_items = Vec::new();

        match client.get(url).send() {
            Ok(resp) => {
                if !resp.status().is_success() {
                    println!("Failed to fetch TechCrunch: {}", resp.status());
                    return Ok(all_items);
                }

                let content = resp.bytes()?;
                let reader = BufReader::new(&content[..]);

                match Channel::read_from(reader) {
                    Ok(channel) => {
                        for item in channel.items().iter().take(5) {
                            if let (Some(title), Some(link)) = (item.title(), item.link()) {
                                all_items.push(NewsItem {
                                    title: title.to_string(),
                                    url: link.to_string(),
                                    source: "TechCrunch".to_string(),
                                    published_date: item.pub_date().unwrap_or("").to_string(),
                                });
                            }
                        }
                    }
                    Err(e) => println!("Error parsing RSS for TechCrunch: {}", e),
                }
            }
            Err(e) => println!("Error fetching TechCrunch: {}", e),
        }

        Ok(all_items)
    }
}
