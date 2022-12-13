use csv::{Reader, StringRecord};
use std::{env, error::Error as stderr, ffi::OsString, fs::File, process};
use std::io::Read;

pub fn readcsv(path: &str) -> Vec<String> {
    let file = File::open(path);
    let mut rdr = csv::Reader::from_reader(file);
    let records = rdr.records();
    
    records
}
