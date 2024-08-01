// ctclsite-rust - CTCL 2020-2024
// File: src/services/mod.rs
// Purpose: Services module
// Created: March 1, 2024
// Modified: July 28, 2024

use actix_web::{web, Error, HttpResponse, Responder, Result};
use crate::{CombinedCfg, mkcontext};

pub async fn services_index(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(&combinedcfg, "services", combinedcfg.services.get("root").unwrap()).unwrap();

    let s = match tmpl.render("sections.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}