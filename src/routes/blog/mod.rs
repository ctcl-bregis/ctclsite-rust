// ctclsite-rust - CTCL 2022-2024
// File: src/routes//blog/mod.rs
// Purpose: Blog module
// Created: March 1, 2024
// Modified: March 3, 2024

use std::collections::{BTreeMap, HashMap};

use actix_web::http::StatusCode;
use actix_web::{
    web, Error, HttpResponse, HttpResponseBuilder, Responder, Result,
};
use crate::{mkcontext, read_file};
use crate::{Sitecfg, Page};

pub async fn blog_index(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<impl Responder, Error> {
    let ctx = mkcontext("blog", "root").unwrap();
    
    let s = match tmpl.render("main/blog_menu.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn blog_post(page: web::Path<String>, tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<impl Responder, Error> {
    let sitecfg: Sitecfg = serde_json::from_str(&read_file(String::from("config/config.json")).unwrap()).unwrap();
    let mut blogcfg: BTreeMap<String, Page> = serde_json::from_str(&read_file(sitecfg.pages.get("blog").unwrap().to_string()).unwrap()).unwrap();
    blogcfg.remove("root");

    if !blogcfg.contains_key(&page.clone()) {
        return Ok(HttpResponse::NotFound().body("Blog post not found"))
    }

    let ctx = mkcontext("blog", &page.clone()).unwrap();

    let s = match tmpl.render("main/blog_post.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err))
    };

    Ok(s)
}