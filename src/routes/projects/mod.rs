// ctclsite-rust - CTCL 2022-2024
// File: src/routes/services/mod.rs
// Purpose: Services module
// Created: March 1, 2024
// Modified: March 6, 2024

use std::collections::HashMap;
use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::{Page, Sitecfg, read_file, mdpath2html};

#[derive(Deserialize, Serialize, Clone)]
struct ProjectsCat {
    title: String,
    desc: String,
    // Project pages have the same format as most other pages
    subpages: IndexMap<String, Page>
}

#[derive(Deserialize, Serialize)]
struct ProjectsCfg {
    // "menu" has the same format as most other pages
    menu: Page,
    cats: IndexMap<String, ProjectsCat>
}

// Function that stores and returns the category associated with a page
fn getpage(page: String) -> Result<Page, std::io::Error> {
    let mut pagemap: HashMap<String, String> = HashMap::new();
    let sitecfg: Sitecfg = serde_json::from_str(&read_file(String::from("config/config.json")).unwrap()).unwrap();
    let projectscfg: ProjectsCfg = serde_json::from_str(&read_file(sitecfg.pages.get("projects").unwrap().to_string()).unwrap()).unwrap();

    for (catkey, catvalue) in projectscfg.cats.clone() {
        for (pagekey, _pagevalue) in catvalue.subpages {
            pagemap.insert(pagekey, catkey.clone());
        }
    }

    let cat = match pagemap.get(&page) {
        Some(cat) => cat,
        None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Page not found".to_string()))
    };

    let pageobject = &projectscfg.cats.get(cat).unwrap().subpages.get(&page).unwrap().clone();

    Ok(pageobject.clone())
}

pub async fn projects_index(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    // Due to the structure of config/projects/config.json, mkcontext cannot be used
    let mut ctx = tera::Context::new();

    let sitecfg: Sitecfg = serde_json::from_str(&read_file(String::from("config/config.json")).unwrap()).unwrap();
    let projectscfg: ProjectsCfg = serde_json::from_str(&read_file(sitecfg.pages.get("projects").unwrap().to_string()).unwrap()).unwrap();
    let themecfg: HashMap<String, String> = serde_json::from_str(&read_file("themes.json".to_string()).unwrap()).unwrap();

    let subpagecfg = projectscfg.menu;

    ctx.insert("menu", &projectscfg.cats);
    ctx.insert("themecolor", &sitecfg.themes.get(&subpagecfg.theme));
    ctx.insert("title", &subpagecfg.title);
    ctx.insert("desc", &subpagecfg.desc);
    ctx.insert("styling", &themecfg.get(&subpagecfg.theme));

    ctx.insert("clientinfojs", &read_file(String::from("static/clientinfo.js")).unwrap());
    ctx.insert("commonjs", &read_file(String::from("static/common.js")).unwrap());

    if !&subpagecfg.favicon.is_none() {
        ctx.insert("favicon", &subpagecfg.favicon.as_ref().unwrap());
    } else {
        let iconname: &str = subpagecfg.theme.as_ref();
        ctx.insert("favicon", &format!("/static/favicons/default_{iconname}.ico"));
    }

    let s = match tmpl.render("projects_menu.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}

pub async fn projects_page(page: web::Path<String>, tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    // Due to the structure of config/projects/config.json, mkcontext cannot be used
    let mut ctx = tera::Context::new();

    let sitecfg: Sitecfg = serde_json::from_str(&read_file(String::from("config/config.json")).unwrap()).unwrap();
    let themecfg: HashMap<String, String> = serde_json::from_str(&read_file("themes.json".to_string()).unwrap()).unwrap();

    let subpagecfg = match getpage(page.to_string()) {
        Ok(pageobject) => pageobject,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Ok(HttpResponse::NotFound().body(format!("{:?}", e))),
            _ => panic!("{}", e)
        }
    };

    ctx.insert("themecolor", &sitecfg.themes.get(&subpagecfg.theme));
    ctx.insert("title", &subpagecfg.title);
    ctx.insert("desc", &subpagecfg.desc);
    ctx.insert("styling", &themecfg.get(&subpagecfg.theme));

    ctx.insert("clientinfojs", &read_file(String::from("static/clientinfo.js")).unwrap());
    ctx.insert("commonjs", &read_file(String::from("static/common.js")).unwrap());

    let rendered = mdpath2html(subpagecfg.content.unwrap()).unwrap();
    ctx.insert("rendered", &rendered);

    if !&subpagecfg.favicon.is_none() {
        ctx.insert("favicon", &subpagecfg.favicon.as_ref().unwrap());
    } else {
        let iconname: &str = subpagecfg.theme.as_ref();
        ctx.insert("favicon", &format!("/static/favicons/default_{iconname}.ico"));
    }

    let s = match tmpl.render("projects_content.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}