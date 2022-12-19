use csv::{self, Error};
//use serde::{Deserialize, Deserializer};
use std::{collections::BTreeMap, fs, convert::TryFrom};
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

pub fn md2html(path: &str) -> Result<String, BTreeMap<String, Error>> {
    dbg!(path);
    let md = fs::read_to_string(path)
        .expect("md2html - File read error");
        
    let content = markdown_to_html(&md, &ComrakOptions::default());
    
    Ok(content)
}

// mkcontext - Make Tera context and prefill values with data in index CSV files
pub fn mkcontext(metapage: &str, subpage: &str) -> Result<(Context, BTreeMap<String, String>), Error> {
    let mut context = Context::new();

    context.insert("active", &metapage);

    // Load pagemeta.csv file
    let metapage_index = csv2bt("./config/pagemeta.csv").unwrap();
    
    // Get corresponding entry
    let mut metapage_entry = BTreeMap::new();
    for entry in metapage_index {
        if entry["page"] == metapage {
            metapage_entry = entry;
            break;
        } 
    }
    // If the for loop completed and did not assign a new value to pagemeta_sub
    if metapage.is_empty() {
        panic!("Page not found");
    }
    
    context.insert("color", &metapage_entry["color"]);
    
    // Get the index file defined for the subpage in pagemeta
    // This is returned with the context for any subpage that does something different with the data
    let subpage_index = csv2bt(&format!("./{}", metapage_entry["index"])).unwrap();
    dbg!(&subpage_index);
    
    let mut subpage_entry = BTreeMap::new();
    for entry in subpage_index {
        if entry["page"] == subpage {
            subpage_entry = entry;
            break;
        } 
    }
    if subpage.is_empty() {
        panic!("Subpage not found");
    }
    
    context.insert("title", &subpage_entry["title"]);
    context.insert("desc", &subpage_entry["desc"]);
    
    Ok((context, subpage_entry))
}

// This part here is why I switched to Rust, the amount of data read from CSV files is large enough that it is important this function is optimized
// Output Tuple item 1: Tables Vector of BTreeMaps of Vectors of BTreeMaps
// Output Tuple item 2: Table column widths
// Output Tuple item 3: Number of entries, unsigned 32-bit integer
pub fn rl_list_gen(list: &str) -> Result<(Vec<BTreeMap<String, Vec<BTreeMap<String, String>>>>, Vec<BTreeMap<String, String>>, u32), Error> {
    let rl_index = csv2bt("./config/ramlist/menu.csv").unwrap();
    
    let tableinfo: Vec<BTreeMap<String, String>> = Vec::new();
    let listindex: Vec<BTreeMap<String, String>> = Vec::new();
    for entry in rl_index {
        if entry["name"] == list {
           let tableinfo = csv2bt(&entry["tabc"]).unwrap();
           let listindex = csv2bt(&entry["indc"]).unwrap();
        }
    }
    let mut entrycount: u32 = 0;
    
    let mut tables = Vec::new();
    for entry in listindex {
    
        let mut entries = csv2bt(&entry["file"]).unwrap(); 
        entrycount = &entrycount + u32::try_from(entries.len()).unwrap();
        
        let mut vendtable = BTreeMap::new();
        vendtable.insert(entry["brand"], entries);
        
        tables.push(vendtable);
    }

    Ok((tables, colwidths, entrycount))
}
