// ctclsite-rust - CTCL 2022-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: May 21, 2024

pub mod routes;
pub mod middleware;

use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error};
use std::result::Result;
use comrak::{markdown_to_html, Options};
use serde::{Deserialize, Serialize};

// Config file terminology:
// config/ - "site"
//   config.json - "sitecfg", "metapage"
//   about/ - "page"
//     config.json - "pagecfg", "subpage"

#[derive(Deserialize, Serialize, Clone)]
pub struct Theme {
    // Main theme color
    color: String,
    // Background color on buttons that are hovered
    bgcolor: String,
    // Used for animations where the theme color transistions to this color
    altcolor: Option<String>
}

// Section struct used for pages made of sections
#[derive(Deserialize, Serialize, Clone)]
pub struct Section {
    theme: String,
    title: String,
    content: String,
    // Value that determines if the section should have the height of the viewport, defaults to true
    fitscreen: Option<bool>,
    bgvid: Option<String>,
    bgimg: Option<String>
}

// Any page that is made up of a single Markdown document
#[derive(Deserialize, Serialize, Clone)]
pub struct ContentPage {
    theme: String,
    title: String,
    desc: String,
    // This is the path to the markdown file before its value is replaced by the rendered document
    content: String,
    // Optional link to favicon 
    favicon: Option<String>,
    // For project and blog pages at the moment
    icon: Option<String>,
    icontitle: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BlogContentPage {
    theme: String,
    title: String,
    desc: String,
    // This is the path to the markdown file before its value is replaced by the rendered document
    content: String,
    // Optional link to favicon 
    favicon: Option<String>,
    // For project and blog pages at the moment
    icon: Option<String>,
    icontitle: Option<String>,
    date: String,
    // To-Do: replace this with actual categorization
    cat: String,
}

// Any page that is made up of sections, including About
#[derive(Deserialize, Serialize, Clone)]
pub struct SectionsPage {
    theme: String,
    title: String,
    desc: Option<String>,
    introduction: Option<String>,
    sections: IndexMap<String, Section>,
    favicon: Option<String>,
    sectionpixfont: Option<bool>,
    // For project and blog pages at the moment
    icon: Option<String>,
    icontitle: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum PageType {
    #[serde(alias = "content")]
    PageTypeContent(ContentPage),
    #[serde(alias = "sections")]
    PageTypeSections(SectionsPage)
}
 
#[derive(Deserialize, Serialize, Clone)]
pub struct LinklistList {
    title: String,
    link: String,
    theme: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LinklistPage {
    theme: String,
    title: String,
    desc: String,
    background: String,
    menu: Vec<LinklistList>,
    favicon: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MetaPage {
    theme: String,
    title: String,
    desc: String,
    favicon: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BlogPage {
    root: MetaPage,
    posts: IndexMap<String, BlogContentPage>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProjectsPageCategory {
    title: String,
    desc: String,
    theme: String,
    subpages: IndexMap<String, PageType>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProjectsPage {
    root: MetaPage,
    cats: IndexMap<String, ProjectsPageCategory>
}

// Get all pages under every category
impl ProjectsPage {
    fn getpages(self) -> IndexMap<String, PageType> {
        let mut pages: IndexMap<String, PageType> = IndexMap::new();
    
        for cat in self.cats.values() {
            for (key, value) in cat.subpages.iter() {
                pages.insert(key.to_owned(), value.to_owned());
            }
        }
        
        pages
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SiteCfg {
    pub aboutcfg: IndexMap<String, PageType>,
    pub linklistcfg: IndexMap<String, LinklistPage>,
    pub blogcfg: BlogPage,
    pub projectscfg: ProjectsPage,
    pub servicescfg: IndexMap<String, SectionsPage>,
    pub themes: HashMap<String, Theme>,
    pub globalcfg: GlobalCfg
}

// config/config.json
#[derive(Deserialize, Serialize, Clone)]
pub struct GlobalCfg {
    pub pages: HashMap<String, String>,
    pub themes: HashMap<String, Theme>,
    pub bindip: String,
    pub bindport: u16,
    pub siteurl: String,
    pub redirects: HashMap<String, String>
}

// -------------------------------------

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path))),
            _ => panic!("Can't read from file: {}, err {}", path.to_owned(), e),
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
    let content = markdown_to_html(&read_file(path).expect("File read error"), &comrak_options);

    Ok(content)
}
