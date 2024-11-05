// ctclsite - CTCL 2019-2024
// File: src/lib.rs
// Purpose: Commonly used functions and types
// Created: November 28, 2022
// Modified: November 4, 2024

use minifier::js;
use comrak::{markdown_to_html, Options};
use image::{Rgb, RgbImage};
use indexmap::IndexMap;
use log::{debug, error, info, warn};
use serde_json::value::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File, DirEntry};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;
use std::result::Result;
use lysine::{Context, Lysine};
use walkdir::WalkDir;

pub mod themes;
pub mod logger;
pub mod page;
pub mod route;

pub use themes::*;
pub use logger::*;
pub use page::loader::*;
pub use route::*;

// To-Do: This file is long, consider splitting some code into modules

// ----------------------------
// serde defaults

pub fn emptystringindexmap() -> IndexMap<String, String> {
    let map: IndexMap<String, String> = IndexMap::new();
    map
}

pub fn emptypagehashmap() -> HashMap<String, Page> {
    let map: HashMap<String, Page> = HashMap::new();
    map
}

pub fn emptythemehashmap() -> HashMap<String, Theme> {
    let map: HashMap<String, Theme> = HashMap::new();
    map
}

pub fn emptyfonthashmap() -> HashMap<String, FontFamily> {
    let map: HashMap<String, FontFamily> = HashMap::new();
    map
}

pub fn emptynavbarlinkhashmap() -> HashMap<String, NavbarLink> {
    let map: HashMap<String, NavbarLink> = HashMap::new();
    map
}

pub fn emptystring() -> String {
    String::new()
}

pub fn emptystringvec() -> Vec<String> {
    let vec: Vec<String> = Vec::new();
    vec
}

pub fn emptyusizevec() -> Vec<usize> {
    let vec: Vec<usize> = Vec::new();
    vec
}

pub fn emptytripleu8() -> [u8; 3] {
    let vec: [u8; 3] = [0u8; 3];
    vec
}

pub fn defaultfalse() -> bool {
    false
}

pub fn defaulttrue() -> bool {
    true
}

// ----------------------------

#[derive(Deserialize, Serialize)]
pub struct NavbarLink {
    title: String,
    link: String
}


#[derive(Deserialize, Serialize)]
pub enum ExtensionFileType {
    #[serde(alias = "binary")]
    Binary,
    // Anything with the "config" type shall not be copied to "static/"
    #[serde(alias = "config")]
    Config,
    #[serde(alias = "image")]
    Image,
    #[serde(alias = "pdf")]
    Pdf,
    #[serde(alias = "text")]
    Text,
    #[serde(alias = "video")]
    Video
}

#[derive(Deserialize, Serialize)]
pub struct URLParameter {
    allowedvalues: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct SiteConfig {
    // IP to bind to, usually either "0.0.0.0" or "127.0.0.1"
    pub bindip: String,
    // Port to bind to
    pub bindport: u16,
    // Website domain, for example: "ctcl.lgbt". Currently used for the "link" meta tag.
    pub sitedomain: String,
    // Directory to scan for font directories;
    pub fontdir: String,
    // Directory to scan for JavaScript files
    pub jsdir: String,
    // Directory to scan for page directories
    pub pagedir: String,
    // Directory to scan for globally-used static files
    pub staticdir: String,
    // Directory to scan for theme directories
    pub themedir: String,
    // Directory to scan for HTML templates
    pub templatedir: String,
    // Theme to default to if a specific theme is not defined. It must refer to an existing theme.
    pub defaulttheme: String,
    // How many "workers" to be deployed by Actix Web
    pub cpus: usize,
    // Map of links to redirect to
    pub redirects: HashMap<String, String>,
    // Links to make available in the navbar
    pub navbar: Vec<NavbarLink>,
    // Maximum level to log to console
    pub debugloglevel: String,
    // Access log configuration data 
    pub logger: LogConfig,
    // Exists solely for debugging purposes. It should be set to "true" in production.
    pub minimizehtml: bool,
    // Exists solely for debugging purposes. It should be set to "true" in production.
    pub minimizecss: bool,
    pub dateformats: HashMap<String, String>,
    // Definition of file types by file extension, used by collectstatic to determine what files to copy and may be used for the upcoming file viewer feature
    pub filetypes: HashMap<String, ExtensionFileType>,
    // Optional: Any extra parameters defined in config.json to be available in Lysine/Tera CSS templates
    pub themevars: Option<HashMap<String, Value>>,
    // Optional: Any extra parameters defined in config.json to be available in HTML templates
    pub uservars: Option<HashMap<String, Value>>,
    // Theme definitions
    pub themes: HashMap<String, Theme>,
    // Skip since pages, themes and fonts should not be defined in config.json
    #[serde(skip_deserializing, default = "emptypagehashmap")]
    pub pages: HashMap<String, Page>,
    #[serde(skip_deserializing, default = "emptyfonthashmap")]
    pub fonts: HashMap<String, FontFamily>,
    // Contents of robots.txt 
    #[serde(skip, default = "emptystring")]
    pub robots: String
}

// Partial config that only has fields for things required to start the webserver to avoid loading all of the pages twice
#[derive(Deserialize, Serialize)]
pub struct PartialSiteConfig {
    pub bindip: String,
    pub bindport: u16,
    pub cpus: usize,
    // Maximum level to log to console
    pub debugloglevel: String
}

pub fn buildjs(sitecfg: &SiteConfig) -> Result<(), Error> {
    match mkdir("static/js/") {
        Ok(_) => (),
        Err(e) =>  return Err(Error::new(ErrorKind::Other, format!("buildjs: Error creating directory \"static/js/\" {e}")))
    }

    match fs::exists(&sitecfg.jsdir) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Err(Error::new(ErrorKind::NotFound, format!("buildjs: Directory {} not found", &sitecfg.jsdir))),
            _ => return Err(Error::new(ErrorKind::Other, format!("buildjs: {e}")))
        }
    }

    let iter = match fs::read_dir(&sitecfg.jsdir) {
        Ok(d) => d,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("buildjs: error reading {}: {e}", &sitecfg.jsdir)))
    };

    let mut dirs: Vec<DirEntry> = Vec::new();
    for dir in iter {
        match dir {
            Ok(rd) => {
                if rd.path().extension().unwrap_or(OsStr::new("")) == "js" {
                    dirs.push(rd);
                } else {
                    // Possibly misleading since files without a ".js" extension could still be JavaScript
                    warn!("buildjs: \"{}\" is not a JavaScript file. Skipping.", rd.path().to_string_lossy())
                }
            },
            Err(_) => continue,
        }
    }

    for dir in dirs {
        let dirstring = dir.path().to_string_lossy().to_string();
        let source = match read_file(&dirstring) {
            Ok(s) => s,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("buildjs: error reading file {dirstring}: {e}")))
        };
        let minified = js::minify(&source);

        let targetpath = format!("static/js/{}", dir.path().file_name().unwrap().to_string_lossy());

        match write_file(&targetpath, &minified.to_string()) {
            Ok(_) => debug!("Minified JavaScript file written to {targetpath}"),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("buildjs: error writing file {targetpath}: {e}")))
        };
    }

    Ok(())
}

pub fn collectstatic(sitecfg: &SiteConfig) -> Result<(), Error> {
    mkdir("static/pages/")?;

    for entry in WalkDir::new(&sitecfg.pagedir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            match entry.path().to_string_lossy().strip_prefix(&sitecfg.pagedir) {
                Some(p) => fs::create_dir_all(format!("static/pages/{}", p))?,
                None => fs::create_dir_all(format!("static/pages/{}", entry.path().to_string_lossy()))?
            }
        }
        
        if entry.path().is_file() {
            match entry.path().extension() {
                Some(fp) => match sitecfg.filetypes.get(&fp.to_string_lossy().into_owned()) {
                    Some(f) => match f {
                        ExtensionFileType::Config => continue,
                        _ => match entry.path().to_string_lossy().strip_prefix(&sitecfg.pagedir) {
                            Some(p) => fs::copy(entry.path(), format!("static/pages/{}", p)).unwrap(),
                            None => fs::copy(entry.path(), format!("static/pages/{}", entry.path().to_string_lossy())).unwrap()
                        }
                    }
                    None => continue
                },
                None => continue
            };
        };
    }

    match fs::read_dir(&sitecfg.staticdir) {
        Ok(d) => {
            for entry in d {
                match entry {
                    Ok(rd) => match fs::copy(rd.path(), format!("static/{}", rd.path().to_string_lossy().strip_prefix(&sitecfg.staticdir).unwrap())) {
                        Ok(_) => (),
                        Err(ce) => return Err(Error::new(ErrorKind::Other, format!("collectstatic failed to copy {} to static/{}, {}", rd.path().to_string_lossy(), rd.path().to_string_lossy(), ce)))
                    }
                    Err(re) => return Err(Error::new(ErrorKind::Other, format!("collectstatic: {re}")))
                }
            }
        },
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("collectstatic: {e}")))
    }

    Ok(())
}

pub fn mkdir(path: &str) -> Result<(), Error> {
    match std::fs::create_dir(path) {
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {
                info!("mkdir: directory {path} already exists, continuing");
                Ok(())
            }
            _ => {
                Err(Error::new(ErrorKind::Other, format!("Error creating directory {path}: {e}")))
            }
        }
        Ok(_) => {
            info!("{path} directory created");
            Ok(())
        }
    }
}

pub fn read_file<T: AsRef<Path>>(path: T) -> Result<String, Error> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(ErrorKind::NotFound, format!("File not found: {}", path.as_ref().to_string_lossy()))),
            // Change from v1: do not panic; return an error instead
            _ => return Err(Error::new(ErrorKind::Other, format!("Error reading from file {}: {e}", path.as_ref().to_string_lossy())))
        }
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;

    Ok(buff)
}

pub fn write_file<T: AsRef<Path>>(path: T, data: &str) -> Result<(), Error> {
    let mut f = match std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Err(Error::new(ErrorKind::Other, format!("write_file: error writing file: {e}"))),
            _ => return Err(Error::new(ErrorKind::Other, format!("write_file: error writing file: {e}")))
        }
    };
    f.write_all(data.as_bytes())?;
    f.flush()?;

    Ok(())
}

pub fn mdpath2html(path: &str) -> Result<String, Error> {
    let mut comrak_options = Options::default();
    comrak_options.render.unsafe_ = true;
    comrak_options.extension.table = true;
    comrak_options.extension.header_ids = None;
    let markdown = match read_file(path) {
        Ok(c) => c,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to render markdown file {path}: {e}")))
    };
    let content = markdown_to_html(&markdown, &comrak_options);

    Ok(content)
}

pub fn loadconfig() -> Result<SiteConfig, Error> {
    let sitecfgdir: &str = &read_file("config.txt").expect("Failed to read from config.txt");
    let mut siteconfig: SiteConfig = serde_json::from_str(&read_file(format!("{}config.json", sitecfgdir)).expect("Failed to read configuration file")).unwrap();

    mkdir("static/")?;

    match buildjs(&siteconfig) {
        Ok(t) => t,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("loadconfig - buildjs: {e}")))
    }

    // Fonts must be loaded before themes are loaded
    siteconfig.fonts = match loadfonts(&siteconfig) {
        Ok(t) => t,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("loadconfig - loadfonts: {e}")))
    }; 
    
    // Themes must be loaded before pages are loaded
    siteconfig.themes = match loadthemes(&siteconfig) {
        Ok(t) => t,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("loadconfig - loadthemes: {e}")))
    };

    match collectstatic(&siteconfig) {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("collectstatic: {e}")))
    };

    // Catch-22: Pages must be loaded to load pages in order to fill in linklist entries with information of a page
    siteconfig.pages = loadpages(&siteconfig)?;
    
    if siteconfig.pages.is_empty() {
        error!("No pages found");
        return Err(Error::new(ErrorKind::NotFound, "No pages found"));
    }
    if siteconfig.themes.is_empty() {
        error!("No themes found");
        return Err(Error::new(ErrorKind::NotFound, "No themes found"));
    }
    
    siteconfig.robots = match read_file(format!("{}robots.txt", sitecfgdir)) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    Ok(siteconfig)
}