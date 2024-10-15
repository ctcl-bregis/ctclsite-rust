// ctclsite - CTCL 2019-2024
// File: src/route/mod.rs
// Purpose: Route module
// Created: October 11, 2024
// Modified: October 13, 2024

use std::collections::HashMap;

use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

use indexmap::IndexMap;
use memcache::Client;
use minify_html::{minify, Cfg};
use lysine::{Context, Lysine};

use log::{debug, info, warn};

use crate::{logaccess, SiteConfig};

pub async fn routepage(req: HttpRequest, page: web::Path<String>, tmpl: web::Data<lysine::Lysine>, sitecfg: web::Data<SiteConfig>, memclient: web::Data<Option<Client>>, query: web::Query<HashMap<String, String>>) -> Result<impl Responder, Error> {
    let mut ctx = Context::new();

    let pagecfg = match page.as_str() {
        "" => sitecfg.pages.get("/").unwrap(),
        _ => match sitecfg.pages.get(page.as_str()) {
            Some(p) => p,
            None => return Ok(HttpResponse::NotFound().body(format!("Page {} not found", page)))
        }
    };

    match memclient.get_ref() {
        Some(m) => {
            logaccess(&sitecfg, req, m)?;
        },
        None => ()
    };

    let theme = match sitecfg.themes.get(&pagecfg.theme) {
        Some(t) => t,
        None => {
            warn!("Theme {} not found, using default", &pagecfg.theme);
            sitecfg.themes.get(&sitecfg.defaulttheme).unwrap()
        }
    };

    ctx.insert("sitedomain", &sitecfg.sitedomain);
    if pagecfg.shownavbar {
        ctx.insert("navbar", &sitecfg.navbar);   
    }

    ctx.insert("title", &pagecfg.title);
    ctx.insert("theme", theme);
    ctx.insert("desc", &pagecfg.desc);
    ctx.insert("keywords", &pagecfg.keywords);
    ctx.insert("favicon", &pagecfg.favicon);
    ctx.insert("headerids", &pagecfg.headerids);
    ctx.insert("rendered", &pagecfg.content);

    match &sitecfg.uservars {
        Some(p) => {
            for (key, value) in p.iter() {
                ctx.insert(key, value);
            }
        },
        None => ()
    }

    // Page-specific variables have a higher precedence over site variables
    match &pagecfg.uservars {
        Some(p) => {
            for (key, value) in p.iter() {
                ctx.insert(key, value);
            }
        },
        None => ()
    }

    // dir name, absolute path
    let mut pathmap: IndexMap<String, String> = IndexMap::new();
    let mut absolute = String::from("/");
    for splitpath in page.split("/") {
        if !splitpath.is_empty() {
            absolute.push_str(&format!("{splitpath}/"));
            pathmap.insert(splitpath.to_owned(), absolute.clone());
        }
    }
    ctx.insert("path", &pathmap);

    let html = match tmpl.render("page.lish", &ctx) {
        Ok(html) => html,
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render template: {:?}", err)))
    };

    let htmlbytes = html.as_bytes();

    let cfg = Cfg {
        // This breaks the webpage when set to false
        do_not_minify_doctype: true,
        ensure_spec_compliant_unquoted_attribute_values: true,
        keep_closing_tags: true,
        keep_html_and_head_opening_tags: true,
        keep_spaces_between_attributes: true,
        keep_comments: false,
        keep_input_type_text_attr: true,
        keep_ssi_comments: false,
        // If any template-related syntax is still in the output, something is quite wrong. Keep for debugging purposes.
        preserve_brace_template_syntax: true,
        preserve_chevron_percent_template_syntax: true,
        // Rendered HTML should have neither CSS or JS. If they do, it is most likely for debugging purposes.
        minify_css: false,
        minify_js: false,
        remove_bangs: false,
        remove_processing_instructions: true
    };

    let htmlmin = match sitecfg.minimizehtml {
        true => minify(htmlbytes, &cfg),
        false => htmlbytes.to_vec(),
    };

    Ok(HttpResponse::Ok().body(htmlmin))
}