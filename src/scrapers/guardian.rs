use super::{NewsItem, Scraper};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::env;

pub struct GuardianScraper;

#[derive(Deserialize)]
struct GuardianResponse {
    response: GuardianResponseBody,
}

#[derive(Deserialize)]
struct GuardianResponseBody {
    results: Vec<GuardianResult>,
}

#[derive(Deserialize)]
struct GuardianResult {
    #[serde(rename = "webTitle")]
    web_title: String,
    #[serde(rename = "webUrl")]
    web_url: String,
    #[serde(rename = "webPublicationDate")]
    web_publication_date: String,
}

impl Scraper for GuardianScraper {
    fn fetch(&self) -> Result<Vec<NewsItem>, Box<dyn Error>> {
        let api_key = env::var("GUARDIAN_API_KEY").unwrap_or_default();
        if api_key.is_empty() {
            println!("Warning: GUARDIAN_API_KEY is not set. Skipping Guardian.");
            return Ok(vec![]);
        }

        let url = format!("https://content.guardianapis.com/search?api-key={}", api_key);
        let client = Client::new();
        let resp = client.get(&url).header("User-Agent", "NewsScraper/1.0").send()?;

        if !resp.status().is_success() {
             println!("Guardian Request failed: {}", resp.status());
             return Ok(vec![]);
        }

        let data: GuardianResponse = resp.json()?;
        
        let items = data.response.results.into_iter().take(5).map(|r| NewsItem {
            title: r.web_title,
            url: r.web_url,
            source: "Guardian".to_string(),
            published_date: r.web_publication_date,
        }).collect();

        Ok(items)
    }
}
