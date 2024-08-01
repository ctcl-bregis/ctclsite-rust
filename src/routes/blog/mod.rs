// ctclsite-rust - CTCL 2020-2024
// File: src/routes/blog/mod.rs
// Purpose: Blog module
// Created: March 1, 2024
// Modified: July 28, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use crate::{CombinedCfg, mkcontext};

pub async fn blog_index(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(&combinedcfg, "blog", combinedcfg.blog.get("root").unwrap()).unwrap();
    
    let s = match tmpl.render("linklist.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn blog_post(page: web::Path<String>, tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let template = match combinedcfg.blog.get(page.as_ref()) {
        Some(pagecfg) => match pagecfg.ptype.as_str() {
            "content" => "content.html",
            "sections" => "sections.html",
            _ => return Ok(HttpResponse::InternalServerError().body("Invalid page type"))
        }
        None => return Ok(HttpResponse::NotFound().body(format!("Page {} not found", page)))
    };

    let ctx = match mkcontext(&combinedcfg, "blog", combinedcfg.blog.get(&page.to_string()).unwrap()) {
        Ok(ctx) => ctx,
        Err(err) => match err.kind() {
            std::io::ErrorKind::InvalidInput => return Ok(HttpResponse::NotFound().body("Page not found")),
            _ => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
        }
    };

    let s = match tmpl.render(template, &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render template: {:?}", err)))
    };

    Ok(s)
}