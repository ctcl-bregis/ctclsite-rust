// ctclsite-rust - CTCL 2020-2024
// File: src/routes/linklist/mod.rs
// Purpose: Module for link lists
// Created: March 13, 2024
// Modified: September 1, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result
};
use crate::{mkcontext, CombinedCfg};

pub async fn linklist(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>, page: web::Path<String>) -> Result<impl Responder, Error> {
    let ctx = match combinedcfg.linklist.get(&page.to_string()) {
        Some(_) => match mkcontext(&combinedcfg, "linklist", combinedcfg.linklist.get(&page.to_string()).unwrap()) {
            Ok(ctx) => ctx,
            Err(err) => match err.kind() {
                std::io::ErrorKind::InvalidInput => return Ok(HttpResponse::NotFound().body("Page not found")),
                _ => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
            }
        },
        None => return Ok(HttpResponse::NotFound().body("Page not found")),
    };

    let s = match tmpl.render("linklist.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err))
    };

    Ok(s)
}