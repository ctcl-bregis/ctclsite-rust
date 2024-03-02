// ctclsite-rust - CTCL 2022-2024
// File: src/blog/mod.rs
// Purpose: Blog module
// Created: March 1, 2024
// Modified: March 1, 2024

use std::collections::HashMap;

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use crate::mkcontext;

pub async fn blog_index(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<impl Responder, Error> {
    let ctx = mkcontext("blog", "root").unwrap();
    
    let s = match tmpl.render("main/blog_menu.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}