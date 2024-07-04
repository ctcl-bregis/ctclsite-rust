// ctclsite-rust - CTCL 2020-2024
// File: src/routes/projects/mod.rs
// Purpose: Projects module
// Created: March 1, 2024
// Modified: June 30, 2024

use actix_web::{web, Error, HttpResponse, Responder, Result};
use crate::{CombinedCfg, mkcontext};

pub async fn projects_index(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(&combinedcfg, "projects", "root").unwrap();
    
    let s = match tmpl.render("linklist.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn projects_page(page: web::Path<String>, tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let template = match combinedcfg.projects.get(page.as_ref()) {
        Some(pagecfg) => match pagecfg.ptype.as_str() {
            "content" => "content.html",
            "sections" => "sections.html",
            _ => return Ok(HttpResponse::InternalServerError().body("Invalid page type"))
        }
        None => return Ok(HttpResponse::NotFound().body(format!("Page {} not found", page)))
    };

    let ctx = match mkcontext(&combinedcfg, "projects", &page) {
        Ok(ctx) => ctx,
        Err(err) => match err.kind() {
            std::io::ErrorKind::InvalidInput => return Ok(HttpResponse::NotFound().body("Page not found")),
            _ => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
        }
    };
    
    let s = match tmpl.render(template, &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}