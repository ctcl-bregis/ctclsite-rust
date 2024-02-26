// ctclsite-rust - CTCL 2020-2024
// File: src/about/mod.rs
// Purpose: About module
// Created: February 26, 2024
// Modified: February 26, 2024

use std::collections::HashMap;

use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    error,
    http::{header::ContentType, StatusCode},
    middleware::{self, ErrorHandlerResponse, ErrorHandlers},
    web, App, Error, HttpResponse, HttpServer, Responder, Result,
};
use tera::Tera;

pub async fn index(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    //ctx.insert("name", name);
    //ctx.insert("text", "Welcome!");

    //let s = tmpl.render("about_main.html", &tera::Context::new()).map_err(|_| error::ErrorInternalServerError("Template error"))?;

    let s = match tmpl.render("main/about_main.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}
