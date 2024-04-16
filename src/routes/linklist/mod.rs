// ctclsite-rust - CTCL 2022-2024
// File: src/routes/linklist/mod.rs
// Purpose: Module for link lists
// Created: March 13, 2024
// Modified: April 10, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result
};
use tera::Context;
use crate::{LinklistPage, SiteCfg};

fn mkcontext(subpage: &LinklistPage) -> Context {
    let mut ctx = Context::new();

    ctx.insert("menu", &subpage.menu);

    ctx
}

pub async fn linklist(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>, page: web::Path<String>) -> Result<impl Responder, Error> {

    let pagecfg = match sitecfg.linklistcfg.get(&page.clone()) {
        Some(pc) => pc,
        None => return Ok(HttpResponse::NotFound().body("Page not found"))
    };

    let ctx = mkcontext(pagecfg);

    let s = match tmpl.render("linklist_main.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err))
    };

    Ok(s)
}