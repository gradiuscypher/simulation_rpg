use reqwest;

use chrono::DateTime;
use chrono::prelude::*;
use scraper::{Html, Selector};
use substring::Substring;


pub struct APoDInfo {
    pub title: String,
    pub desc: String
}

pub fn get_date() -> String {
    let local: DateTime<Local> = Local::now();
    return local.format("%A, %e %B %Y").to_string();
}

pub async fn get_apod_info() -> APoDInfo {
    let uri = "https://apod.nasa.gov/apod/";
    let resp = reqwest::get(uri).await.unwrap();
    
    let body = resp.text().await.unwrap();
    let fragment = Html::parse_document(&body);
    
    let selector_title = Selector::parse("center + center > b:first-child").unwrap();
    let title = fragment.select(&selector_title)
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .to_string();

    let selector_desc = Selector::parse("center + center + p").unwrap();
    let desc = fragment.select(&selector_desc)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()
        .join("");

    let desc = desc.replace("\n", "");
    let desc = desc.replace("Explanation: ", "");
    let desc = desc.substring(0, 250).to_string();

    let read_more: &str = "...\n[read more »](https://apod.nasa.gov/apod/)";
    let desc = desc + read_more;

    return APoDInfo {
        title,
        desc
    }
}
