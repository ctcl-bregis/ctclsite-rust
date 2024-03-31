// ctclsite-rust - CTCL 2022-2024
// File: src/routes/services/mod.rs
// Purpose: Services module
// Created: March 1, 2024
// Modified: March 29, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use crate::{mkcontext, SiteCfg};

pub async fn projects_index(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(sitecfg.get_ref().to_owned(), "projects", "root").unwrap();

    let s = match tmpl.render("projects_menu.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}

pub async fn projects_page(page: web::Path<String>, tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let projectscfg = &sitecfg.projectscfg.clone().topagecfg().pages;

    if !projectscfg.contains_key(&page.clone()) {
        return Ok(HttpResponse::NotFound().body("Page not found"))
    }

    let ctx = mkcontext(sitecfg.get_ref().to_owned(), "projects", &page.to_string()).unwrap();

    let s = match tmpl.render("projects_content.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}