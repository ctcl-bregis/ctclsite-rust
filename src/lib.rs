// ctclsite-rust - CTCL 2020-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: September 5, 2024

use std::io::Error;
use serde_json::value::Value;

pub struct SiteConfig {
    bindip: String,
    bindport: u16,
    pages: Value
}



pub fn loadconfig() -> Result<SiteConfig, Error> {


    Ok()
}