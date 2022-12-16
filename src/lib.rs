use csv::{self, Error};
//use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::fs;

use comrak::{markdown_to_html, ComrakOptions};

type Record = BTreeMap<String, String>;

pub fn csv2hm(path: &str) -> Result<Vec<BTreeMap<String, String>>, Error> {
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
        .expect("File read error");
        
    let content = markdown_to_html(&md, &ComrakOptions::default());
    
    Ok(content)
}
