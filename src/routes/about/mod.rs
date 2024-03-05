// ctclsite-rust - CTCL 2022-2024
// File: src/routes/about/mod.rs
// Purpose: About module
// Created: February 26, 2024
// Modified: March 4, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use crate::mkcontext;

pub async fn about_index(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let ctx = mkcontext("about", "root").unwrap();

    let s = match tmpl.render("about_main.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn about_privacy(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let ctx = mkcontext("about", "privacy").unwrap();
    
    let s = match tmpl.render("about_md_pp.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn about_licensing(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let ctx = mkcontext("about", "licensing").unwrap();

    let s = match tmpl.render("about_md.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}
