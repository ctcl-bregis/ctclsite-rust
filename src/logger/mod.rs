// ctclsite - CTCL 2019-2024
// File: src/logger/mod.rs
// Purpose: Log-related types and functions
// Created: September 21, 2024
// Modified: October 15, 2024

use std::{collections::HashMap, fs::{remove_file, rename, File}, io::Error};
use actix_web::HttpRequest;
use chrono::Utc;
use indexmap::IndexMap;
use log::{debug, warn};
use memcache::Client;
use serde::{Serialize, Deserialize};
use flate2::{write::GzEncoder, Compression};
use uuid::Uuid;
//use tar::Builder;

use crate::{mkdir, read_file, write_file, SiteConfig};

// For both ServerLogType and ClientLogType: the key is the name of the database field
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "header")]
pub enum ServerLogType {
    #[serde(alias = "ip")]
    IPAddress,
    #[serde(alias = "header")]
    Header(String),
    // Same as header but the key-value is passed to template contexts. Used, for example, set the page theme to a dark variant if Sec-CH-Prefers-Color-Scheme is "dark". 
    #[serde(alias = "headertmplvar")]
    HeaderTmplVar(String)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "key")]
pub enum ClientLogType {
    // Data represented by a JSON Key to be inserted into the database
    #[serde(alias = "client")]
    Client(String),
    // Data represented by a JSON Key from the client (browser) to be hashed server-side before insertion into log
    #[serde(alias = "clienthash")]
    ClientHashed(String)
}

#[derive(Deserialize, Serialize)]
pub struct LogConfig {
    // Enable/disable logging entirely
    pub enable: bool,
    // URL for the memcache server
    pub memcache: String,
    // Path to the SQLite3 database to use
    //pub logpath: String,
    // The string to be sent as "Accept-CH" in the HTTP response header if logging is enabled
    pub clienthints: String,
    // Headers to log to the access log
    pub serverlog: IndexMap<String, ServerLogType>,
    // Data to log to the access log
    pub clientlog: IndexMap<String, ClientLogType>
}

pub struct LogEntry {
    pub headermap: HashMap<String, String>
}

pub fn logaccess(sitecfg: &SiteConfig, req: HttpRequest, memclient: &Client) -> Result<String, Error> {
    let uuid = Uuid::new_v4().to_string();
    let _ = memclient.set(&uuid, false, 120);

    let mut logmap: HashMap<String, String> = HashMap::new();

    for (key, header) in &sitecfg.logger.serverlog {
        match header {
            ServerLogType::IPAddress => match req.peer_addr() {
                Some(ip) => {
                    logmap.insert(key.to_string(), ip.to_string());
                }
                None => {
                    logmap.insert(key.to_string(), String::from(""));
                }
            },
            ServerLogType::Header(h) => {
                match req.headers().get(h) {
                    Some(h) => match h.to_str() {
                        Ok(hs) => {
                            logmap.insert(key.to_string(), hs.to_string());
                        },
                        Err(_) => {
                            logmap.insert(key.to_string(), String::from(""));
                        }
                    },
                    None => {
                        debug!("Header {h} not sent by client");
                        logmap.insert(key.to_string(), String::from(""));
                    }
                };                
            },
            ServerLogType::HeaderTmplVar(h) => {
                match req.headers().get(h) {
                    Some(h) => match h.to_str() {
                        Ok(hs) => {
                            logmap.insert(key.to_string(), hs.to_string());
                        },
                        Err(_) => {
                            logmap.insert(key.to_string(), String::from(""));
                        }
                    },
                    None => {
                        debug!("Header {h} not sent by client");
                        logmap.insert(key.to_string(), String::from(""));
                    }
                };
            }
        }
    }

    debug!("UUID inserted into memcache: {uuid}");

    let mut keys = String::new();
    let mut values = String::new();
    for (key, value) in logmap {
        keys.push_str(&format!(r"{}, ", key));
        values.push_str(&format!(r"'{}', ", value));
    }

    let query = format!("INSERT INTO log ({}) VALUES({});", keys.strip_suffix(", ").unwrap(), values.strip_suffix(", ").unwrap());
    let conn = sqlite::open("log/log.db").unwrap();
    conn.execute(query).unwrap();

    Ok(uuid)
    
}

// Must always be called on server startup
pub fn loggersetup(sitecfg: &SiteConfig) -> Result<(), Error> {
    mkdir("log/").unwrap();

    let mut schema = String::new();

    schema.push_str("CREATE TABLE IF NOT EXISTS log (\n");
    for field in sitecfg.logger.serverlog.keys() {
        schema.push_str(&format!("    {field} TEXT,\n"));
    };
    for field in sitecfg.logger.clientlog.keys() {
        schema.push_str(&format!("    {field} TEXT,\n"));
    };
    schema = schema.strip_suffix(",\n").unwrap().to_string();
    schema.push_str(");\n");
    let schema_hash = sha256::digest(&schema);

    let current_schema_hash = match read_file("log/schema_sha256.txt") {
        Ok(h) => h,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => String::new(),
            _ => return Err(e),
        }
    };

    let fmt = "%Y%m%d_%H%M%S%z";
    let ts = Utc::now().format(fmt).to_string().replace("+", "p").replace("-", "n");

    if current_schema_hash.is_empty() {
        write_file("log/schema.sql", &schema).unwrap();
        write_file("log/schema_sha256.txt", &schema_hash).unwrap();
    } else if schema_hash != current_schema_hash {
        warn!("Database table configuration has changed, creating backup");
        remove_file("log/schema_sha256.txt")?;
        let tar_gz = File::create(format!("log/log_{}.tar.gz", ts))?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_path("log/log.db")?;
        rename("log/log.db", format!("log/log_{}.db", ts))?;
        write_file("log/schema.sql", &schema)?;
        write_file("log/schema_sha256.txt", &schema_hash)?;
    }

    let conn = sqlite::open("log/log.db").unwrap();
    conn.execute(schema).unwrap();

    Ok(())

}