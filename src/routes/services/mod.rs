// ctclsite-rust - CTCL 2022-2024
// File: src/services/mod.rs
// Purpose: Services module
// Created: March 1, 2024
// Modified: May 20, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use indexmap::IndexMap;
use tera::Context;
use crate::{SiteCfg, SectionsPage, Section, mdpath2html};

fn mkcontext(sitecfg: &SiteCfg, subpage: &SectionsPage) -> Context {
    let mut ctx = Context::new();

    ctx.insert("title", &subpage.title);
    let defaultfavicon = &format!("/static/favicons/default_{}.ico", &subpage.theme);

    ctx.insert("favicon", match &subpage.favicon {
        Some(link) => link,
        None => defaultfavicon
    });
    ctx.insert("themename", &subpage.theme);
    ctx.insert("themecolor", &sitecfg.themes.get(&subpage.theme).unwrap().color);
    ctx.insert("desc", &subpage.desc);
    ctx.insert("sectionpixfont", &subpage.sectionpixfont);

    let mut renderedsections: IndexMap<String, Section> = IndexMap::new();
    let mut isvideo: bool = false;
    for section in subpage.sections.iter() {
        let mut renderedsection = section.1.clone();

        renderedsection.content = mdpath2html(&renderedsection.content, false).unwrap();
        
        if renderedsection.bgvid.is_some() {
            isvideo = true;
        }

        if renderedsection.fitscreen.is_none() {
            renderedsection.fitscreen = Some(true);
        }

        renderedsections.insert(section.0.to_string(), renderedsection);
    }

    if subpage.introduction.is_some() {
        let renderedintro = mdpath2html(subpage.introduction.as_ref().unwrap(), false).unwrap();
        ctx.insert("introduction", &renderedintro);
    }

    ctx.insert("video", &isvideo);
    ctx.insert("sections", &renderedsections);

    ctx
}

pub async fn services_index(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(sitecfg.get_ref(), sitecfg.servicescfg.get("root").unwrap());

    let s = match tmpl.render("sections.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}