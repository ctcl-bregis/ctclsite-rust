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

    let pages = csv2bt("./config/pagemeta.csv").unwrap();
    
    let mut pagebt = BTreeMap::new();
    for entry in pages {
        if entry["page"] == page {
            pagebt = entry;
            break;
        }
        
    }
    if pagebt.is_empty() {
        panic!("Page not found");
    }
    
    context.insert("title", &String::from(format!("{} - {}", pagebt["title"], "CrazyblocksTechnologies Computer Laboratories")));
    context.insert("active", &String::from(&pagebt["title"]));
    context.insert("desc", &String::from(format!("{} - {}", pagebt["desc"], "CrazyblocksTechnologies Computer Laboratories")));

    Ok(context)
}
    
