// ctclsite-rust - CTCL 2020-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: July 15, 2024

pub mod routes;

use indexmap::IndexMap;
use tera::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error};
use std::result::Result;
use comrak::{markdown_to_html, Options};
use serde::{Serialize, Deserialize};

fn true_default() -> bool {
    true
}

fn false_default() -> bool {
    false
}

fn empty_string() -> String {
    "".to_string()
}


fn empty3u8() -> [u8; 3] {
    [0u8; 3]
}

fn basestring() -> String {
    "base".to_string()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
    // Main theme color
    color: String,
    #[serde(default = "empty3u8")]
    colorrgb: [u8; 3],
    // Text color on theme color
    fgcolor: String,
    #[serde(default = "empty3u8")]
    fgcolorrgb: [u8; 3],
    // Theme-specific styling, "base" by default
    #[serde(default = "basestring")]
    css: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Var {
    value: String,
    #[serde(alias = "type")]
    vtype: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Font {
    family: String,
    path: String,
    weight: String,
    style: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StylingCfg {
    vars: HashMap<String, Var>,
    fonts: Vec<Font>,
    css: HashMap<String, String>,
    themes: HashMap<String, Theme>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Section {
    theme: String,
    title: String,
    content: String,
    // Value that determines if the section should have the height of the viewport, defaults to true
    #[serde(default = "true_default")]
    fitscreen: bool,
    bgvid: Option<String>,
    bgimg: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Category {
    title: String,
    theme: String
}

// Any page that is made up of sections, including About
#[derive(Serialize, Deserialize, Clone)]
pub struct Page {
    // Base
    #[serde(rename = "type")]
    ptype: String,
    // "link" should be assigned the name of the key if empty
    #[serde(default = "empty_string")]
    link: String,
    theme: String,
    title: String,
    #[serde(default = "empty_string")]
    desc: String,
    #[serde(default = "empty_string")]
    keywords: String,
    favicon: Option<String>,
    #[serde(default = "empty_string")]
    icon: String,
    #[serde(default = "empty_string")]
    icontitle: String,
    cat: Option<String>,
    startdate: Option<String>,
    enddate: Option<String>,
    #[serde(default = "true_default")]
    shownavbar: bool,
    // Sections only
    sections: Option<IndexMap<String, Section>>,
    // Content only
    content: Option<String>,
    // Linklist only
    // WARNING - This is much different from "cat"; "cats" stores what categories are available to a linklist page
    cats: Option<IndexMap<String, Category>>,
    menu: Option<Vec<String>>,
    #[serde(default = "false_default")]
    menugroupbycat: bool
}

#[derive(Serialize, Deserialize, Clone)]
// config/config.json
pub struct PageCfgPaths {
    pub about: String,
    pub blog: String,
    pub linklist: String,
    pub projects: String,
    pub ramlist: String,
    pub services: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SiteCfg {
    pub bindip: String,
    pub bindport: u16,
    pub siteurl: String,
    pub styling: StylingCfg,
    pub pagecfgpaths: PageCfgPaths,
    pub redirects: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CombinedCfg {
    pub sitecfg: SiteCfg,
    pub about: HashMap<String, Page>,
    pub linklist: HashMap<String, Page>,
    pub blog: HashMap<String, Page>,
    pub projects: HashMap<String, Page>,
    pub services: HashMap<String, Page>,
//    pub ramlist: HashMap<String, Page>
}

// -------------------------------------

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path))),
            _ => panic!("Error reading from file {}: {}", path.to_owned(), e),
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

pub fn fill_empty_links(map: HashMap<String, Page>) -> HashMap<String, Page> {
    let mut newmap: HashMap<String, Page> = HashMap::new();
    for (key, value) in map.into_iter() {
        let mut newpage: Page = value.clone();
        if newpage.link.is_empty() {
            newpage.link.clone_from(&key);
        }
        newmap.insert(key, newpage);
    }
    newmap
}

pub fn get_combined_cfg() -> Result<CombinedCfg, Error> {
    let sitecfg: SiteCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();

    let about: HashMap<String, Page> = fill_empty_links(serde_json::from_str(&read_file(&sitecfg.pagecfgpaths.about).unwrap()).unwrap());
    let blog: HashMap<String, Page> = fill_empty_links(serde_json::from_str(&read_file(&sitecfg.pagecfgpaths.blog).unwrap()).unwrap());
    let linklist: HashMap<String, Page> = fill_empty_links(serde_json::from_str(&read_file(&sitecfg.pagecfgpaths.linklist).unwrap()).unwrap());
    let projects: HashMap<String, Page> = fill_empty_links(serde_json::from_str(&read_file(&sitecfg.pagecfgpaths.projects).unwrap()).unwrap());
    let services: HashMap<String, Page> = fill_empty_links(serde_json::from_str(&read_file(&sitecfg.pagecfgpaths.services).unwrap()).unwrap());
//    let ramlist: HashMap<String, Page> = serde_json::from_str(&read_file(&sitecfg.pagecfgpaths.ramlist).unwrap()).unwrap();

    Ok(CombinedCfg {
        sitecfg,
        about,
        linklist,
        blog,
        projects,
        services,
//        ramlist,
    })
}


pub fn mkcontext(sitecfg: &CombinedCfg, page: &str, subpage: &str) -> Result<Context, Error> {
    let mut ctx = Context::new();

    let pagecfg = match page {
        "about" => &sitecfg.about,
        "blog" => &sitecfg.blog,
        "linklist" => &sitecfg.linklist,
        "projects" => &sitecfg.projects,
        "services" => &sitecfg.services,
        _ => return Err(Error::new(std::io::ErrorKind::NotFound, format!("Page {} not found", page))),
    };

    let subpage = match pagecfg.get(subpage) {
        Some(subpage) => subpage,
        None => return Err(Error::new(std::io::ErrorKind::NotFound, format!("Page {} not found", page))),
    };

    if subpage.ptype == "link" {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Page is a link and not a page"))
    }

    let pagetheme = match sitecfg.sitecfg.styling.themes.get(&subpage.theme) {
        Some(theme) => theme,
        None => return Err(Error::new(std::io::ErrorKind::NotFound, "Theme not found"))
    };

    let favicon: String = match &subpage.favicon {
        Some(favicon) => favicon.clone(),
        None => format!("/static/favicons/default_{}.ico", &subpage.theme)
    };

    ctx.insert("link", &format!("{}{}", &sitecfg.sitecfg.siteurl, &subpage.link));
    ctx.insert("themename", &subpage.theme);
    ctx.insert("themecolor", &pagetheme.color);
    ctx.insert("title", &subpage.title);
    ctx.insert("desc", &subpage.desc);
    ctx.insert("keywords", &subpage.keywords);
    ctx.insert("favicon", &favicon);
    ctx.insert("shownavbar", &subpage.shownavbar);
    
    if subpage.sections.is_some() {
        let mut sections: IndexMap<String, Section> = IndexMap::new();
        for (name, data) in &subpage.sections.clone().unwrap() {
            let mut newdata = data.clone();
            newdata.content = mdpath2html(&data.content, false).unwrap();

            sections.insert(name.to_string(), newdata);
        }

        ctx.insert("sections", &sections);
    }
    
    if subpage.content.is_some() {
        let newcontent = mdpath2html(subpage.content.as_ref().unwrap(), true).unwrap();

        ctx.insert("content", &newcontent)
    }

    if subpage.cats.is_some() {
        ctx.insert("cats", subpage.cats.as_ref().unwrap());
    }

    if subpage.menu.is_some() {
        let mut newmenu: Vec<Page> = Vec::new();
        for page in subpage.menu.as_ref().unwrap() {
            newmenu.push(pagecfg.get(&page.clone()).unwrap().clone());
        }
        ctx.insert("menu", &newmenu);
        ctx.insert("menugroupbycat", &subpage.menugroupbycat);
    }


    Ok(ctx)
}