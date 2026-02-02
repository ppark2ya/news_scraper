use reqwest;
use scraper::{Html, Selector};

fn main() {
    // 1. HTTP 요청
    let body = reqwest::blocking::get("https://www.nytimes.com")
        .unwrap()
        .text()
        .unwrap();

    // 2. 파싱
    let document = Html::parse_document(&body);
    let selector = Selector::parse("h2").unwrap();

    for element in document.select(&selector) {
        println!("{}", element.text().collect::<Vec<_>>().join(""));
    }
}