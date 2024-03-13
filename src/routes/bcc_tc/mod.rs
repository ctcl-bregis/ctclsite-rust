// ctclsite-rust - CTCL 2022-2024
// File: src/routes/bcc_tc/mod.rs
// Purpose: About module
// Created: March 13, 2024
// Modified: March 13, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result
};
use crate::{mkcontext, SiteCfg};

pub async fn bcctc_index(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(sitecfg.get_ref().to_owned(), "bcc_tc", "root").unwrap();

    let s = match tmpl.render("bcc_tc.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err))
    };

    Ok(s)
}