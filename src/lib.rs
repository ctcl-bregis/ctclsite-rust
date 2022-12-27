use csv::{self, Error};
//use ::serde::{Deserialize, Deserializer};
use indexmap::IndexMap;
//use serde_derive::{Deserialize, Serialize};
use std::{fs, convert::TryFrom};
use tera::Context;
use comrak::{markdown_to_html, ComrakOptions};

type Record = IndexMap<String, String>;

// Input: file path, Output: Vector of IndexMap objects
pub fn csv2im(path: &str) -> Result<Vec<IndexMap<String, String>>, Error> {
    let mut records = Vec::new();
    let rdr = csv::Reader::from_path(path);
    
    for result in rdr.unwrap().deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn md2html(path: &str) -> Result<String, IndexMap<String, Error>> {
    dbg!(&path);
    let md = fs::read_to_string(path)
        .expect("md2html - File read error");
        
    let content = markdown_to_html(&md, &ComrakOptions::default());
    
    Ok(content)
}

// mkcontext - Make Tera context and prefill values with data in index CSV files
pub fn mkcontext(metapage: &str, subpage: &str) -> Result<(Context, IndexMap<String, String>), Error> {
    let mut context = Context::new();

    context.insert("active", &metapage);

    // Load pagemeta.csv file
    let metapage_index = csv2im("./config/pagemeta.csv").unwrap();
    
    // Get corresponding entry
    let mut metapage_entry = IndexMap::new();
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
    
    // Get the index file defined for the subpage in pagemeta
    // This is returned with the context for any subpage that does something different with the data
    let subpage_index = csv2im(&format!("./{}", metapage_entry["index"])).unwrap();
    
    let mut subpage_entry = IndexMap::new();
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
    context.insert("color", &subpage_entry["color"]);
    
    Ok((context, subpage_entry))
}

// This part here is why I switched to Rust, the amount of data read from CSV files is large enough that it is important this function is optimized
// Output Tuple item 1: Tables Vector of IndexMaps of Vectors of IndexMaps
// Output Tuple item 2: Table column widths
// Output Tuple item 3: Number of entries, unsigned 32-bit integer
pub fn rl_list_gen(list: &str) -> Result<(IndexMap<String, Vec<IndexMap<String, String>>>, IndexMap<String, IndexMap<String, String>>, u32), Error> {
    let rl_index = csv2im("./config/ramlist/menu.csv").unwrap();
    
    let mut tableinfo: Vec<IndexMap<String, String>> = Vec::new();
    let mut listindex: Vec<IndexMap<String, String>> = Vec::new();
    for entry in rl_index {
        if entry["name"] == list {
           tableinfo = csv2im(&format!("./config/ramlist/lists/{}", &entry["tabc"])).unwrap();
           listindex = csv2im(&format!("./config/ramlist/lists/{}", &entry["indc"])).unwrap();
        }
    }
    
    let mut entrycount: u32 = 0;
    
    let mut tables = IndexMap::new();
    for entry in listindex {
    
        let entries = csv2im(&format!("./config/ramlist/lists/{}/{}", list, &entry["file"])).unwrap();
        entrycount = &entrycount + u32::try_from(entries.len()).unwrap();
        
        tables.insert(entry["brand"].clone(), entries);
    }

    let mut colwidths = IndexMap::new();
    for entry in tableinfo {
        let mut tmpim = IndexMap::new();
        tmpim.insert(String::from("title"), entry["title"].clone());
        tmpim.insert(String::from("width"), entry["width"].clone());
        // ydata name left over from when the website used YAML with Python...
        colwidths.insert(entry["ydata"].clone(), tmpim);
    }
    
    // Should be disabled during development unless it is absolutely needed due to the massive amount of data this IndexMap contains, this will add several seconds to the page loading time with or without "--release"
    //dbg!(&tables);
    
    //dbg!(&colwidths);
    
    Ok((tables, colwidths, entrycount))
}
