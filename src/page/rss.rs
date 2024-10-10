// ctclsite - CTCL 2019-2024
// File: src/page/rss.rs
// Purpose: RSS feed generator module
// Created: October 1, 2024
// Modified: October 9, 2024

use std::time::SystemTime;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};
use crate::Page;

pub fn crateversion() -> String {
    format!("ctclsite-rust {}", env!("CARGO_PKG_VERSION"))
}

#[derive(Debug, Serialize, PartialEq)]
pub struct RSSItem {
    title: String,
    link: String,
    description: String,
    #[serde(rename = "pubDate")]
    pubdate: String, 
    guid: String
}

#[derive(Debug, Serialize, PartialEq)]
pub struct RSSChannel {
    title: String,
    link: String,
    description: String,
    language: String,
    #[serde(rename = "pubDate")]
    pubdate: String,
    lastbuilddate: String,
    #[serde(skip_deserializing)]
    generator: String, 
    item: Vec<RSSItem>
}


#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rss")]
pub struct RSS {
    channel: RSSChannel
}

pub fn page2rss(page: &Page) -> RSS {
    let pagecfg = page.clone();

    let time = Utc::now().to_rfc3339();

    let items: Vec<RSSItem> = Vec::new();
    

    let channel = RSSChannel {
        title: pagecfg.title,
        link: pagecfg.link,
        description: pagecfg.desc,
        language: "en-US".to_owned(),
        pubdate: pagecfg.startdate,
        lastbuilddate: time,
        generator: crateversion(),
        item: items,
    };

    RSS {
        channel: channel
    }

}