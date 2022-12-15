use std::{fs::File};
use csv::{self, Error, Reader};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

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
