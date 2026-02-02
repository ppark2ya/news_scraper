use super::{NewsItem, Scraper};
use rss::Channel;
use std::error::Error;
use reqwest::blocking::get;
use std::io::BufReader;

pub struct KoreaScraper;

impl Scraper for KoreaScraper {
    fn fetch(&self) -> Result<Vec<NewsItem>, Box<dyn Error>> {
        let feeds = vec![
            ("MK", "https://www.mk.co.kr/rss/40300001/"), 
            ("Yonhap", "https://www.yna.co.kr/rss/news.xml"), 
        ];

        let mut all_items = Vec::new();

        for (source_name, url) in feeds {
            match get(url) {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        println!("Failed to fetch {}: {}", source_name, resp.status());
                        continue;
                    }
                    
                    let content = resp.bytes()?;
                    let reader = BufReader::new(&content[..]);
                    
                    match Channel::read_from(reader) {
                        Ok(channel) => {
                            for item in channel.items().iter().take(3) {
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
                },
                Err(e) => println!("Error fetching {}: {}", source_name, e),
            }
        }

        Ok(all_items)
    }
}
