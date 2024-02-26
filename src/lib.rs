// ctclsite-rust - CTCL 2020-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: February 26, 2024

pub mod routes;
pub mod logger;

use std::io::Error;
use serde::{Deserialize, Serialize};
use serde_json::Result;

// Config file terminology:
// config/
//   config.json: "sitecfg", "metapage"
//   about/
//     config.json: "pagecfg", "subpage"



//pub fn mkcontext(metapage: &str, subpage: &str) -> Result<tera::Context, Error> {
//    todo!();
//}


