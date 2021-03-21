use chrono::prelude::*;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use serde::Deserialize;
use substring::Substring;

pub struct APoDInfo {
    pub title: String,
    pub img: String,
    pub desc: String
}

pub struct XkcdInfo {
    pub title: String,
    pub img: String,
    pub transcript: String,
    pub alt: String,
    pub num: u64,
    pub date: String
}

#[derive(Debug, Deserialize)]
pub struct Comic {
    month: String,
    num: u64,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String
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

    let selector_a = Selector::parse("body > center:nth-child(1) > p:nth-child(3) > a").unwrap();
    let a = fragment.select(&selector_a)
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .to_string()
        .replace("\n", "");
    
    let re = Regex::new(r#"<img.+?src=["'](.+?)["'].*?>"#).unwrap();
    let cap = re.captures(&a).unwrap();
    let src: &str = &cap[1].to_string();
    let img = uri.to_owned() + src;

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
        img,
        desc
    }
}

pub async fn get_xkcd_info() -> XkcdInfo {
    let uri = "http://xkcd.com/info.0.json ";
    let resp = reqwest::get(uri).await.unwrap();

    let body = resp.json::<Comic>().await.unwrap();
    let dt = chrono::Local.ymd(body.year.parse::<i32>().unwrap(), body.month.parse::<u32>().unwrap(), body.day.parse::<u32>().unwrap());

    return XkcdInfo {
        title: body.title,
        img: body.img,
        transcript: body.transcript,
        alt: body.alt,
        num: body.num,
        date: dt.format("%A, %e %B %Y").to_string()
    }
}
