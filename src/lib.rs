// ctclsite-rust - CTCL 2022-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: March 21, 2024

pub mod routes;

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

// configuration file structs
#[derive(Deserialize, Serialize, Clone)]
pub struct Theme {
    color: String,
    fgcolor: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Section {
    title: String,
    id: Option<String>,
    content: String,
    bgvid: Option<String>,
    bgimg: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Linklistlink {
    title: String,
    link: String,
    theme: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SubPageCfg {
    theme: String,
    title: String,
    desc: String,
    content: Option<String>,
    favicon: Option<String>,
    sections: Option<Vec<Section>>,
    background: Option<String>,
    menu: Option<Vec<Linklistlink>>,
    icon: Option<String>,
    icontitle: Option<String>,
    cat: Option<String>,
    date: Option<String>,
    js: Option<bool>,
    navbar: Option<bool>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PageCfg {
    pages: IndexMap<String, SubPageCfg>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProjectCatCfg {
    title: String,
    desc: String,
    subpages: IndexMap<String, SubPageCfg>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProjectsPageCfg {
    root: SubPageCfg,
    cats: IndexMap<String, ProjectCatCfg>
}

// Add function to convert ProjectsPagesCfg to a PageCfg with all of the pages under projects
impl ProjectsPageCfg {
    fn topagecfg(self) -> PageCfg {
        let mut pages: IndexMap<String, SubPageCfg> = IndexMap::new();
        pages.insert("root".to_string(), self.root);
        for cat in self.cats.values() {
            for subpage in cat.subpages.iter() {
                pages.insert(subpage.0.clone(), subpage.1.clone());
            }
        }
        let res: PageCfg = PageCfg { pages };
        res
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SiteCfg {
    pub aboutcfg: PageCfg,
    pub bcctccfg: PageCfg,
    pub blogcfg: PageCfg,
    pub projectscfg: ProjectsPageCfg,
    pub servicescfg: PageCfg,
    pub themes: HashMap<String, Theme>,
    pub themes_css: HashMap<String, String>,
    pub globalcfg: GlobalCfg
}

// config/config.json
#[derive(Deserialize, Serialize, Clone)]
pub struct GlobalCfg {
    pub pages: HashMap<String, String>,
    pub themes: HashMap<String, Theme>,
    pub bindip: String,
    pub bindport: u16,
    pub siteurl: String
}

// -------------------------------------

pub fn read_file(path: String) -> Result<String, Error> {
    let tmppath = path.clone();

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", tmppath))),
            _ => panic!("Can't read from file: {}, err {}", tmppath.to_owned(), e),
        }
    };
    //let mut file = File::open(path).expect("Failed to open file");
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

pub fn mdpath2html(path: String) -> Result<String, Error> {
    let mut comrak_options = Options::default();
    comrak_options.render.unsafe_ = true;
    let content = markdown_to_html(&read_file(path).expect("File read error"), &comrak_options);

    Ok(content)
}

// Function that prefills the Tera context
pub fn mkcontext(sitecfg: SiteCfg, metapage: &str, subpage: &str) -> Result<tera::Context, Error> {
    let mut ctx = tera::Context::new();
    let mut comrak_options = Options::default();
    comrak_options.render.unsafe_ = true;

    let pagecfg = match metapage {
        "about" => sitecfg.aboutcfg,
        "bcc_tc" => sitecfg.bcctccfg,
        "blog" => sitecfg.blogcfg,
        "projects" => sitecfg.projectscfg.clone().topagecfg(),
        "services" => sitecfg.servicescfg,
        _ => return Err(Error::new(std::io::ErrorKind::NotFound, "Invalid page".to_string()))
    };
    let themecfg = sitecfg.themes;
    let themecsscfg = sitecfg.themes_css;
    let subpagecfg = match pagecfg.pages.get(subpage) {
        Some(subpage) => subpage,
        None => return Err(Error::new(std::io::ErrorKind::NotFound, "Page not found".to_string()))
    };

    // JavaScript is only disabled explicitly; a page entry without the js field would have JavaScript enabled by default
    if subpagecfg.js == Some(false) {
        ctx.insert("js", &false)
    } else {
        ctx.insert("js", &true)
    }

    // navbar follows the same rule
    if subpagecfg.navbar == Some(false) {
        ctx.insert("navbar", &false)
    } else {
        ctx.insert("navbar", &true)
    }

    // Another hack that should be removed soon
    if metapage == "about" && subpage == "root" {
        ctx.insert("url", &format!("https://{}/", sitecfg.globalcfg.siteurl));
    } else if metapage == "about" && subpage != "root" {
        ctx.insert("url", &format!("https://{}/{}/", sitecfg.globalcfg.siteurl, subpage));
    } else if subpage == "root" {
        ctx.insert("url", &format!("https://{}/{}/", sitecfg.globalcfg.siteurl, metapage));
    } else {
        ctx.insert("url", &format!("https://{}/{}/{}/", sitecfg.globalcfg.siteurl, metapage, subpage));
    }

    ctx.insert("themecolor", &themecfg.get(&subpagecfg.theme).unwrap().color);
    ctx.insert("title", &subpagecfg.title);
    ctx.insert("desc", &subpagecfg.desc);

    if metapage == "bcc_tc" {
        ctx.insert("menu", &subpagecfg.menu);
        ctx.insert("background", &subpagecfg.background);
    } else if metapage == "projects" {
        ctx.insert("menu", &sitecfg.projectscfg.cats);
    }
    ctx.insert("styling", &themecsscfg.get(&subpagecfg.theme).unwrap());

    ctx.insert("vidsjs", &read_file(String::from("static/vids.js")).unwrap());
    ctx.insert("clientinfojs", &read_file(String::from("static/clientinfo.js")).unwrap());
    ctx.insert("commonjs", &read_file(String::from("static/common.js")).unwrap());

    if !&subpagecfg.content.is_none() {
        let mdpath = subpagecfg.content.as_ref().unwrap();
        let rendered = markdown_to_html(&read_file(mdpath.to_owned()).unwrap(), &comrak_options);

        ctx.insert("rendered", &rendered);
    }

    if !&subpagecfg.favicon.is_none() {
        ctx.insert("favicon", &subpagecfg.favicon.as_ref().unwrap());
    } else {
        let iconname: &str = subpagecfg.theme.as_ref();
        ctx.insert("favicon", &format!("/static/favicons/default_{iconname}.ico"));
    }

    if !&subpagecfg.sections.is_none() {
        let sections = subpagecfg.sections.as_ref().unwrap();
        let mut newsections: Vec<Section> = vec![];

        for section in sections {

            let mdpath = section.content.clone();
            let rendered = mdpath2html(mdpath.to_owned()).unwrap();

            let newsection: Section = Section { title: section.title.clone(), id: section.id.clone(), content: rendered, bgvid: section.bgvid.clone(), bgimg: section.bgimg.clone() };
            newsections.push(newsection);
        }

        ctx.insert("sections", &newsections);
    }

    // Blog is a special case
    if metapage == "blog" {
        if subpage == "root" {
            let mut posts = pagecfg.pages.clone();
            posts.shift_remove("root");
            
            ctx.insert("posts", &posts)
        } else {
            let mdpath = subpagecfg.content.as_ref().unwrap();
            let rendered = mdpath2html(mdpath.to_owned()).unwrap();
            ctx.insert("rendered", &rendered);
        }

    }

    Ok(ctx)
}


