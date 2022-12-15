use csv::{self, Error};
//use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs;

use comrak::{markdown_to_html, ComrakOptions};

type Record = HashMap<String, String>;

pub fn csv2hm(path: &str) -> Result<Vec<HashMap<String, String>>, Error> {
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
        .expect("Should have been able to read the file");
        
    let content = markdown_to_html(&md, &ComrakOptions::default());
    
    Ok(content)
}
