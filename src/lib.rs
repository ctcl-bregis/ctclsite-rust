use csv::{self, Error};
//use serde::{Deserialize, Deserializer};
use std::{collections::BTreeMap, fs};
use tera::Context;
use comrak::{markdown_to_html, ComrakOptions};

type Record = BTreeMap<String, String>;

// Input: file path, Output: Vector of BTreeMap objects
pub fn csv2bt(path: &str) -> Result<Vec<BTreeMap<String, String>>, Error> {
    let mut records = Vec::new();
    let rdr = csv::Reader::from_path(path);
    
    for result in rdr.unwrap().deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn md2html(path: &str) -> Result<String, Error> {
    let md = fs::read_to_string(path)
        .expect("md2html - File read error");
        
    let content = markdown_to_html(&md, &ComrakOptions::default());
    
    Ok(content)
}

pub fn mkcontext(page: &str) -> Result<Context, Error> {
    let mut context = Context::new();

    let pagemeta = csv2bt("./config/pagemeta.csv").unwrap();
    
    let mut pagemetasrc = BTreeMap::new();
    for entry in pagemeta {
        if entry["page"] == page {
            pagemetasrc = entry;
            break;
        } 
    }
    if pagemetasrc.is_empty() {
        panic!("Page not found");
    }
    
    Ok(context)
}
    
