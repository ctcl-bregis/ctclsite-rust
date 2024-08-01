// ctclsite-rust - CTCL 2020-2024
// File: src/routes/about/mod.rs
// Purpose: About module
// Created: February 26, 2024
// Modified: July 28, 2024

use actix_web::{web, Error, HttpResponse, Responder, Result};
use crate::{mkcontext, CombinedCfg};

pub async fn about_index(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(&combinedcfg, "about", combinedcfg.about.get("root").unwrap()).unwrap();

    let s = match tmpl.render("sections.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn about_privacy(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(&combinedcfg, "about", combinedcfg.about.get("privacy").unwrap()).unwrap();
    
    let s = match tmpl.render("content.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn about_licensing(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(&combinedcfg, "about", combinedcfg.about.get("licensing").unwrap()).unwrap();

    let s = match tmpl.render("content.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}
