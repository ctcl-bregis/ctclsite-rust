// ctclsite - CTCL 2019-2024
// File: src/page/loader.rs
// Purpose: Page configuration loader
// Created: October 1, 2024
// Modified: October 13, 2024

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkFullCustom {
    pub link: String,
    pub theme: String,
    pub startdate: String,
    pub enddate: String,
    pub title: String,
    pub desc: String,
    pub icon: String,
    pub icontitle: String,
    pub cat: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkFull {
    pub page: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkTitleOnlyCustom {
    pub title: String,
    pub theme: String,
    pub link: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkTitleOnly {
    pub page: String
}

// Text-only
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkTitleText {
    pub text: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum LinkType {
    #[serde(rename(serialize = "full", deserialize = "full"))]
    Full(LinkFull),
    #[serde(rename(serialize = "fullcustom", deserialize = "fullcustom"))]
    FullCustom(LinkFullCustom),
    #[serde(rename(serialize = "titleonly", deserialize = "titleonly"))]
    TitleOnly(LinkTitleOnly),
    #[serde(rename(serialize = "titleonlycustom", deserialize = "titleonlycustom"))]
    TitleOnlyCustom(LinkTitleOnlyCustom),
    #[serde(rename(serialize = "titletext", deserialize = "titletext"))]
    TitleText(LinkTitleText)
}

// Weird name, I know
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentContent {
    pub boxed: bool,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    pub title: String,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    pub theme: String,
    #[serde(default = "defaultfalse")]
    pub fitscreen: bool,
    pub content: String,
    #[serde(skip_deserializing, default = "emptystring")]
    pub rendered: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentLinklist {
    pub boxed: bool,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    pub title: String,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    pub theme: String,
    #[serde(default = "defaultfalse")]
    pub fitscreen: bool,
    pub links: Vec<LinkType>,
    #[serde(skip_deserializing, default = "emptystring")]
    pub rendered: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(alias = "linklist")]
    Linklist(ContentLinklist),
    #[serde(alias = "content")]
    Content(ContentContent)
}


// All pages are "Sections" now
// To replicate a "Content" page from ctclsite-rust v1, there would be one section with "boxed" 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    #[serde(skip_deserializing, default = "emptystring")]
    pub link: String,
    // -- Commonly defined fields --
    // Title of the page that is displayed in the <title></title> tag, the page header and meta tags
    pub title: String,
    // If not defined, this value is to be replaced with the default theme specified in <config root>/config.json
    #[serde(default = "emptystring")]
    pub theme: String,
    // Currently, this can be anything. For example: "2024", "????", "February 22, 2022"
    #[serde(default = "emptystring")]
    pub startdate: String,
    // Currently, this can be anything. For example: "2024", "????", "February 22, 2022"
    // Unused if startdate is undefined or empty
    #[serde(default = "emptystring")]
    pub enddate: String,
    // Description to show in meta tags and Linklist entries
    #[serde(default = "emptystring")]
    pub desc: String,
    // Icon to show on Linklist entries; not the page favicon
    #[serde(default = "emptystring")]
    pub icon: String,
    // Mouseover text for icons. Unused if icon is not defined or empty.
    #[serde(default = "emptystring")]
    pub icontitle: String,
    // Keywords for pages
    #[serde(default = "emptystringvec")]
    pub keywords: Vec<String>,
    // Category that a page has in a Linklist entry
    #[serde(default = "emptystring")]
    pub cat: String,
    // If not defined, the HTML template should use the default favicon that corresponds with the page theme
    #[serde(default = "emptystring")]
    pub favicon: String,
    // Should be set to "false" with pages like About and Services that are made up of boxed sections
    #[serde(default = "defaulttrue")]
    pub headerids: bool,
    #[serde(default = "defaulttrue")]
    pub shownavbar: bool,
    // Definitions of content to be processed. This value should not be used directly in templates.
    pub content: IndexMap<String, Content>,
    // Any extra parameters defined in configuration files to be available in HTML templates
    pub uservars: Option<HashMap<String, Value>>
}

// path arg is used to append the path to the content file
pub fn loadcontent(pagemap: &HashMap<String, Page>, page: &Page, path: &str, tmpl: &lysine::Lysine) -> Result<IndexMap<String, Content>, Error> {
    let mut pagecontent: IndexMap<String, Content> = IndexMap::new();

    for (id, content) in page.content.clone().into_iter() {
        let newcontent = match content {
            Content::Linklist(c) => {
                let mut ctx = Context::new();
                ctx.insert("pages", pagemap);
                ctx.insert("links", &c.links);

                let rendered = match tmpl.render("types/linklist.lish", &ctx) {
                    Ok(c) => c,
                    Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error when rendering page {path}: {:?}", e)))
                };

                let mut newcontent = c.clone();
                newcontent.rendered = rendered;

                Content::Linklist(newcontent)
            },
            Content::Content(c) => {
                let mut ctx = Context::new();
                // &path should already end with "/" so there should not be another "/" between the path and file name
                let content = match mdpath2html(&format!("{}{}", &path, &c.content)) {
                    Ok(c) => c,
                    Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error when rendering page {path}: {e}")))
                };

                ctx.insert("content", &content);

                let rendered = match tmpl.render("types/content.lish", &ctx) {
                    Ok(c) => c,
                    Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error when rendering page {path}: {e}")))
                };

                let mut newcontent = c.clone();
                newcontent.rendered = rendered;

                Content::Content(newcontent)
            }
        };

        pagecontent.insert(id.to_owned(), newcontent);
    }

    Ok(pagecontent)
}

pub fn loadpages(sitecfg: &SiteConfig) -> Result<HashMap<String, Page>, Error> {
    let mut pagemap: HashMap<String, Page> = HashMap::new();

    let tmpl = Lysine::new(&format!("{}/**/*.lish", sitecfg.templatedir)).unwrap();
    
    for entry in WalkDir::new(&sitecfg.pagedir).into_iter().filter_map(|e| e.ok()) {
        if !entry.path().is_dir() {
            continue
        }
        let pagejson = format!("{}/page.json", entry.path().to_string_lossy());
        let pagepath = format!("{}/", &entry.path().to_string_lossy());
        let pagejsonpath = Path::new(&pagejson);

        if fs::exists(format!("{pagepath}.placeholder")).unwrap_or(false) {
            debug!("loadpages: directory {pagepath} has .placeholder, skipping.");
        } else if pagejsonpath.exists() {
            if !pagejsonpath.is_file() {
                warn!("loadpages: {} is not a file", pagejsonpath.to_string_lossy());
                continue;
            }
            let pageconfigraw = &read_file(&pagejson).unwrap();
            match serde_json::from_str::<Page>(pageconfigraw) {
                Ok(pageconfig) => {
                    let mut pagecfg = pageconfig.clone();

                    pagecfg.link = format!("https://{}/{}", sitecfg.sitedomain, pagepath);

                    if pagecfg.favicon.is_empty() {
                        pagecfg.favicon = format!("/static/favicons/default_{}.ico", pagecfg.theme)
                    }
                    pagemap.insert(pagepath.strip_prefix(&sitecfg.pagedir).unwrap().to_string(), pagecfg);
                }
                Err(err) => {
                    warn!("loadpages: {pagejson} failed to deserialize: {err}");
                    continue;
                }
            };
        } else {
            warn!("loadpages: page.json not found in {}", entry.path().to_string_lossy());
        }
    }

    // Catch-22: Pages must be loaded to load pages in order to fill in linklist entries with information of a page
    let mut newpagemap: HashMap<String, Page> = HashMap::new();
    for (pagepath, pagecfg) in pagemap.iter() {
        let mut newpagecfg: Page = pagecfg.clone();

        newpagecfg.content = match loadcontent(&pagemap, pagecfg, &format!("{}{}", &sitecfg.pagedir, pagepath), &tmpl) {
            Ok(c) => c,
            Err(e) => {
                warn!("loadpages: {e}");
                continue    
            }
        };

        newpagemap.insert(pagepath.to_string(), newpagecfg);
    }

    Ok(newpagemap)
}