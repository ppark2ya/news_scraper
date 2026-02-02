use super::{NewsItem, Scraper};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::env;

pub struct NytScraper;

#[derive(Deserialize)]
struct NytResponse {
    results: Vec<NytResult>,
}

#[derive(Deserialize)]
struct NytResult {
    title: String,
    url: String,
    published_date: String,
}

impl Scraper for NytScraper {
    fn fetch(&self) -> Result<Vec<NewsItem>, Box<dyn Error>> {
        let api_key = env::var("NYT_API_KEY").unwrap_or_default();
        if api_key.is_empty() {
             println!("Warning: NYT_API_KEY is not set. Skipping NYT.");
             return Ok(vec![]);
        }

        // Using home top stories
        let url = format!("https://api.nytimes.com/svc/topstories/v2/home.json?api-key={}", api_key);
        let client = Client::new();
        // User-Agent is often required
        let resp = client.get(&url).header("User-Agent", "NewsScraper/1.0").send()?;
        
        if !resp.status().is_success() {
            println!("NYT Request failed: {}", resp.status());
            return Ok(vec![]);
        }

        let data: NytResponse = resp.json()?;
        
        let items = data.results.into_iter().take(5).map(|r| NewsItem {
            title: r.title,
            url: r.url,
            source: "NYT".to_string(),
            published_date: r.published_date,
        }).collect();

        Ok(items)
    }
}
