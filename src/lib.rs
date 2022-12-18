use csv::{self, Error};
//use serde::{Deserialize, Deserializer};
use std::{collections::BTreeMap, fs};
use tera::Context;
use comrak::{markdown_to_html, ComrakOptions};

type Record = BTreeMap<String, String>;

// Input: file path, Output: Vector of BTreeMap objects
pub fn csv2bt(path: &str) -> Result<Vec<BTreeMap<String, String>>, Error> {
    let mut records = Vec::new();
    dbg!(path);
    let rdr = csv::Reader::from_path(path);
    
    for result in rdr.unwrap().deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn md2html(path: &str) -> Result<String, BTreeMap, Error> {
    let md = fs::read_to_string(path)
        .expect("md2html - File read error");
        
    let content = markdown_to_html(&md, &ComrakOptions::default());
    
    Ok(content)
}

pub fn mkcontext(page: &str) -> Result<Context, Error> {
    let mut context = Context::new();

    // Load pagemeta.csv file
    let pagemeta = csv2bt("./config/pagemeta.csv").unwrap();
    
    // Get corresponding entry
    let mut pagemeta_sub = BTreeMap::new();
    for entry in pagemeta {
        if entry["page"] == page {
            pagemeta_sub = entry;
            break;
        } 
    }
    // If the for loop completed and did not assign a new value to pagemeta_sub
    if pagemeta_sub.is_empty() {
        panic!("Page not found");
    }
    
    // Get the index file defined for the subpage in pagemeta
    let index_sub = csv2bt(&format!("./{}", pagemeta_sub["index"]));
    dbg!(index_sub);
    
    
    
    
    
    Ok(context, index_sub)
}
    
