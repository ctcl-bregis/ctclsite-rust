// ctclsite-rust - CTCL 2020-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: September 8, 2024

use comrak::{markdown_to_html, Options};
use indexmap::IndexMap;
use serde_json::value::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::Path;
use std::result::Result;
use tera::Context;
use log::{error, info, warn};

use walkdir::WalkDir;

// ----------------------------
// serde defaults

pub fn emptypagehashmap() -> HashMap<String, Page> {
    let map: HashMap<String, Page> = HashMap::new();
    map
}

pub fn emptythemehashmap() -> HashMap<String, Theme> {
    let map: HashMap<String, Theme> = HashMap::new();
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

// ----------------------------

#[derive(Deserialize, Serialize)]
pub struct NavbarLink {
    title: String,
    link: String
}

#[derive(Deserialize, Serialize)]
pub struct FontVariant {
    // Path to font relative to font directory
    path: String,
    style: String,
    weight: String
}

#[derive(Deserialize, Serialize)]
pub struct FontFamily {
    // Internal name
    name: String,
    // Display name
    displayname: String,
    variants: HashMap<String, FontVariant>
}

#[derive(Deserialize, Serialize)]
pub struct Theme {
    // Theme color
    color: String,
    // Color of text that on a background of the theme color
    fgcolor: String,
    defaultfont: String,
    
}

#[derive(Deserialize, Serialize)]
pub struct SiteConfig {
    pub bindip: String,
    pub bindport: u16,
    pub sitedomain: String,
    pub pagedir: String,
    pub themedir: String,
    pub cpus: usize,
    pub redirects: HashMap<String, String>,
    pub navbar: Vec<NavbarLink>,
    // Skip since pages and themes should not be defined in config.json
    #[serde(skip, default = "emptypagehashmap")]
    pub pages: HashMap<String, Page>,
    #[serde(skip, default = "emptythemehashmap")]
    pub themes: HashMap<String, Theme>
}

// Partial config that only has fields for things required to start the webserver to avoid loading all of the pages twice
#[derive(Deserialize, Serialize)]
pub struct PartialSiteConfig {
    pub bindip: String,
    pub bindport: u16,
    pub cpus: usize
}

// -- Page Type Definitions --

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkCustom {
    link: String,
    theme: String,
    title: String,
    icon: String,
    icontitle: String,
    cat: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkCustomTitleOnly {
    link: String,
    theme: String,
    title: String
}

// Have the ability to specifiy if a link uses data (theme, description, etc.) from a page configuration
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum LinkType {
    // Use data from a page entry
    Page(String),
    // Define custom link
    Custom(LinkCustom),
    PageTitleOnly(String),
    CustomTitleOnly(LinkCustomTitleOnly)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    title: String,
    content: String,
    // Unused if boxed is false
    theme: String,
    boxed: bool,
    fitscreen: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageSections {
    // -- Commonly defined fields --
    pub title: String,
    // If not defined, this value should be replaced with the default theme specified in <config root>/config.json
    #[serde(default = "emptystring")]
    pub theme: String,
    #[serde(default = "emptystring")]
    pub startdate: String,
    #[serde(default = "emptystring")]
    pub enddate: String,
    #[serde(default = "emptystring")]
    pub desc: String,
    #[serde(default = "emptystring")]
    pub icon: String,
    // Unused if icon is not defined or empty
    #[serde(default = "emptystring")]
    pub icontitle: String,
    #[serde(default = "emptystringvec")]
    pub keywords: Vec<String>,
    #[serde(default = "emptystring")]
    pub cat: String,
    // If not defined, the HTML template should use the default favicon that corresponds with the page theme
    #[serde(default = "emptystring")]
    pub favicon: String,
    // -- PageSections specific fields --
    pub sections: Vec<Section>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PageContent {
    // -- Commonly defined fields --
    pub title: String,
    // If not defined, this value should be replaced with the default theme specified in <config root>/config.json
    #[serde(default = "emptystring")]
    pub theme: String,
    #[serde(default = "emptystring")]
    pub startdate: String,
    #[serde(default = "emptystring")]
    pub enddate: String,
    #[serde(default = "emptystring")]
    pub desc: String,
    #[serde(default = "emptystring")]
    pub icon: String,
    // Unused if icon is not defined or empty
    #[serde(default = "emptystring")]
    pub icontitle: String,
    #[serde(default = "emptystringvec")]
    pub keywords: Vec<String>,
    #[serde(default = "emptystring")]
    pub cat: String,
    // If not defined, the HTML template should use the default favicon that corresponds with the page theme
    #[serde(default = "emptystring")]
    pub favicon: String,
    // -- PageContent specific fields --
    pub content: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PageLinklist {
    // -- Commonly defined fields --
    pub title: String,
    // If not defined, this value should be replaced with the default theme specified in <config root>/config.json
    #[serde(default = "emptystring")]
    pub theme: String,
    #[serde(default = "emptystring")]
    pub startdate: String,
    #[serde(default = "emptystring")]
    pub enddate: String,
    #[serde(default = "emptystring")]
    pub desc: String,
    #[serde(default = "emptystring")]
    pub icon: String,
    // Unused if icon is not defined or empty
    #[serde(default = "emptystring")]
    pub icontitle: String,
    #[serde(default = "emptystringvec")]
    pub keywords: Vec<String>,
    #[serde(default = "emptystring")]
    pub cat: String,
    // If not defined, the HTML template should use the default favicon that corresponds with the page theme
    #[serde(default = "emptystring")]
    pub favicon: String,
    // -- PageLinklist specific fields --
    pub links: Vec<LinkType>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Page {
    #[serde(alias = "content")]
    Content(PageContent),
    #[serde(alias = "linklist")]
    Linklist(PageLinklist),
    #[serde(alias = "sections")]
    Sections(PageSections)
}

pub fn loadpages(sitecfg: &SiteConfig) -> HashMap<String, Page> {
    let mut pagemap: HashMap<String, Page> = HashMap::new();
    
    let dir = sitecfg.pagedir;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            let pagejson = format!("{}/page.json", entry.path().to_string_lossy());
            let pagejsonpath = Path::new(&pagejson);
            if pagejsonpath.exists() {
                if pagejsonpath.is_file() {
                    let pageconfigraw = &read_file(&pagejson).unwrap();
                    let mut pageconfig: Page = match serde_json::from_str(pageconfigraw) {
                        Ok(tc) => match tc {
                            Page::Content(c) => {
                                let content: String = match &mdpath2html(&c.content, true) {
                                    Ok(c) => c.to_string(),
                                    Err(e) => {
                                        warn!("loadpages: failed to read content file {0}: {e}", c.content);
                                        continue;
                                    }
                                };

                                let theme = match c.theme

                                let favicon;
                                if c.favicon.is_empty() {
                                    favicon = format!("/static/favicons/theme/{}_icon.ico", theme);
                                }
                                Page::Content(
                                    PageContent {
                                        title: c.title,
                                        theme,
                                        startdate: c.startdate,
                                        enddate: c.enddate,
                                        desc: c.desc,
                                        icon: c.icon,
                                        icontitle: c.icontitle,
                                        keywords: c.keywords,
                                        cat: c.cat,
                                        favicon: c.favicon,
                                        content,
                                    }
                                )
                            }, 
                            Page::Linklist(c) => {
                                
                            },
                            Page::Sections(c) => {
    
                            }
                        }
                        Err(err) => {
                            warn!("loadpages: {pagejson} failed to deserialize: {err}");
                            continue;
                        }
                    };
                    pagemap.insert(pagejson.strip_prefix(dir).unwrap().to_string(), pageconfig);
                } else {
                    warn!("loadpages: page.json in {} is a directory", entry.path().to_string_lossy());
                }
            } else {
                warn!("loadpages: page.json not found in {}", entry.path().to_string_lossy());
            }
        }
    }
    pagemap
}

// Basically the same thing
pub fn loadthemes(dir: &str) -> HashMap<String, Theme> {
    let mut thememap: HashMap<String, Theme> = HashMap::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            let themejson = format!("{}/theme.json", entry.path().to_string_lossy());
            let themejsonpath = Path::new(&themejson);
            if themejsonpath.exists() {
                if themejsonpath.is_file() {
                    let themeconfigraw = &read_file(&themejson).unwrap();
                    match serde_json::from_str(themeconfigraw) {
                        Ok(tc) => thememap.insert(themejson.strip_prefix(dir).unwrap().to_string(), tc),
                        Err(err) => {
                            warn!("loadthemes: {themejson} failed to deserialize: {err}");
                            continue;
                        }
                    };
                } else {
                    warn!("loadthemes: theme.json in {} is a directory", entry.path().to_string_lossy());
                }
            } else {
                warn!("loadthemes: theme.json not found in {}", entry.path().to_string_lossy());
            }
        }
    }
    thememap
}


pub fn mkdir(path: &str) -> Result<(), Error> {
    match std::fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(ErrorKind::NotFound, format!("File {} not found", path))),
            // Change from v1: do not panic; return an error instead
            _ => return Err(Error::new(ErrorKind::Other, format!("Error reading from file {}: {}", path.to_owned(), e))),
        }
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

pub fn mdpath2html(path: &str, headerids: bool) -> Result<String, Error> {
    let mut comrak_options = Options::default();
    comrak_options.render.unsafe_ = true;
    comrak_options.extension.table = true;
    if headerids {
        comrak_options.extension.header_ids = Some("".to_string());
    }
    let markdown = &read_file(path)?;
    let content = markdown_to_html(markdown, &comrak_options);

    Ok(content)
}

pub fn loadsections(sections: &Vec<Section>) -> Vec<Section> {
    let mut newsections: Vec<Section> = Vec::new();

    for section in sections {
        let newsection = Section {
            title: section.title.clone(),
            content: mdpath2html(&section.content, true).unwrap(),
            theme: section.theme.clone(),
            boxed: section.boxed,
            fitscreen: section.fitscreen
        };
        newsections.push(newsection);
        
    }
    newsections
}

pub fn loadconfig() -> Result<SiteConfig, Error> {
    let mut siteconfig: SiteConfig = serde_json::from_str(&read_file("ctclsite-config/config.json").unwrap()).unwrap();
    // Themes must be loaded before pages are loaded
    siteconfig.themes = loadthemes(&siteconfig.themedir);
    siteconfig.pages = loadpages(&siteconfig);

    if siteconfig.pages.is_empty() {
        error!("No pages found");
        return Err(Error::new(ErrorKind::NotFound, "No pages found"));
    }
    if siteconfig.themes.is_empty() {
        error!("No themes found");
        return Err(Error::new(ErrorKind::NotFound, "No themes found"));
    }

    Ok(siteconfig)
}