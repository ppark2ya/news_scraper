use std::error::Error;

#[derive(Debug)]
pub struct NewsItem {
    pub title: String,
    pub url: String,
    pub source: String,
    pub published_date: String,
}

pub trait Scraper {
    fn fetch(&self) -> Result<Vec<NewsItem>, Box<dyn Error>>;
}

pub mod nyt;
pub mod guardian;
pub mod korea;
pub mod cnbc;
pub mod techcrunch;
