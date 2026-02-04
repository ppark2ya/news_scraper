use super::{NewsItem, Scraper};
use rss::Channel;
use std::error::Error;
use reqwest::blocking::Client;
use std::io::BufReader;

pub struct CnbcScraper;

impl Scraper for CnbcScraper {
    fn fetch(&self) -> Result<Vec<NewsItem>, Box<dyn Error>> {
        let feeds = vec![
            ("CNBC Business", "https://www.cnbc.com/id/10001147/device/rss/rss.html"),
            ("CNBC Economy", "https://www.cnbc.com/id/20910258/device/rss/rss.html"),
        ];

        let client = Client::builder()
            .user_agent("Mozilla/5.0 (compatible; NewsBot/1.0)")
            .build()?;

        let mut all_items = Vec::new();

        for (source_name, url) in feeds {
            match client.get(url).send() {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        println!("Failed to fetch {}: {}", source_name, resp.status());
                        continue;
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
                                        source: source_name.to_string(),
                                        published_date: item.pub_date().unwrap_or("").to_string(),
                                    });
                                }
                            }
                        }
                        Err(e) => println!("Error parsing RSS for {}: {}", source_name, e),
                    }
                }
                Err(e) => println!("Error fetching {}: {}", source_name, e),
            }
        }

        Ok(all_items)
    }
}
