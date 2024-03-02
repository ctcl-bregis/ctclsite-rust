// ctclsite-rust - CTCL 2022-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: March 1, 2024

pub mod routes;
pub mod logger;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error};
use std::result::Result;
use log::debug;
use comrak::{markdown_to_html, Options};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Config file terminology:
// config/ - "site"
//   config.json - "sitecfg", "metapage"
//   about/ - "page"
//     config.json - "pagecfg", "subpage"

// configuration file structs
#[derive(Deserialize, Serialize)]
struct Theme {
    color: String,
    fgcolor: String
}

#[derive(Deserialize, Serialize)]
struct Navitem {
    navtype: String,
    app: String,
    title: String,
    link: String,
    float: String,
}

#[derive(Deserialize, Serialize)]
struct Sitecfg {
    main_scss: String,
    lite_scss: String,
    themes: HashMap<String, Theme>,
    navbar: Vec<Navitem>,
    pages: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Section {
    title: String,
    id: Option<String>,
    content: String,
    bgvid: Option<String>,
    bgimg: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Linklistlink {
    title: String,
    link: String,
    theme: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Blogpost {
    id: String,
    content: String,
    icon: String,
    icontitle: String,
    theme: String,
    cat: String,
    date: String,
    title: String,
    desc: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Page {
    theme: String,
    title: String,
    desc: String,
    posts: Option<Vec<Blogpost>>,
    content: Option<String>,
    sections: Option<Vec<Section>>,
    menu: Option<Vec<Linklistlink>>
}

#[derive(Deserialize, Serialize, Debug)]
struct Pagecfg {
    pages: HashMap<String, Page>
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
    let content = markdown_to_html(&read_file(path).expect("File read error"), &Options::default());

    Ok(content)
}

// Function that prefills the Tera context
pub fn mkcontext(metapage: &str, subpage: &str) -> Result<tera::Context, Error> {
    let mut ctx = tera::Context::new();
    let mut comrak_options = Options::default();
    comrak_options.render.unsafe_ = true;
    
    let themecfg: HashMap<String, String> = serde_json::from_str(&read_file("themes.json".to_string()).unwrap()).unwrap();

    // TODO: find out how to load the configurations on start instead of every time this function is called
    let sitecfg: Sitecfg = serde_json::from_str(&read_file(String::from("config/config.json")).unwrap()).unwrap();

    let mut pagecfg: Pagecfg = Pagecfg { pages: HashMap::new() };

    pagecfg.pages = if metapage == "about" {
        let lookup: HashMap<String, Page> = serde_json::from_str(&read_file(sitecfg.pages.get("about").unwrap().to_string()).unwrap()).unwrap();
        lookup
    } else if metapage == "blog" {
        let lookup: HashMap<String, Page> = serde_json::from_str(&read_file(sitecfg.pages.get("blog").unwrap().to_string()).unwrap()).unwrap();
        lookup
    } else if metapage == "projects" {
        let lookup: HashMap<String, Page> = serde_json::from_str(&read_file(sitecfg.pages.get("projects").unwrap().to_string()).unwrap()).unwrap();
        lookup
    } else if metapage == "services" {
        let lookup: HashMap<String, Page> = serde_json::from_str(&read_file(sitecfg.pages.get("services").unwrap().to_string()).unwrap()).unwrap();
        lookup
    } else {
        return Err(Error::new(std::io::ErrorKind::NotFound, "Invalid page".to_string()))
    };

    let subpagecfg = pagecfg.pages.get(subpage);

    ctx.insert("themecolor", &sitecfg.themes.get(&subpagecfg.unwrap().theme));
    ctx.insert("title", &subpagecfg.unwrap().title);
    ctx.insert("desc", &subpagecfg.unwrap().desc);
    ctx.insert("menu", &subpagecfg.unwrap().menu);
    ctx.insert("styling", &themecfg.get(&subpagecfg.unwrap().theme));

    if !&subpagecfg.unwrap().content.is_none() {
        let mdpath = subpagecfg.unwrap().content.as_ref().unwrap();
        let rendered = markdown_to_html(&read_file(mdpath.to_owned()).unwrap(), &comrak_options);

        ctx.insert("rendered", &rendered);
    }

    if !&subpagecfg.unwrap().sections.is_none() {
        let sections = subpagecfg.unwrap().sections.as_ref().unwrap();
        let mut newsections: Vec<Section> = vec![];

        for section in sections {

            let mdpath = section.content.clone();
            let rendered = markdown_to_html(&read_file(mdpath).unwrap(), &comrak_options);

            let newsection: Section = Section { title: section.title.clone(), id: section.id.clone(), content: rendered, bgvid: section.bgvid.clone(), bgimg: section.bgimg.clone() };
            newsections.push(newsection);
        }

        ctx.insert("sections", &newsections);
    }

    // Blog is a special case
    if metapage == "blog" && subpage == "root" {

    }

    Ok(ctx)
}


