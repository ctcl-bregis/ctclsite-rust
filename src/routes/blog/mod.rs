// ctclsite-rust - CTCL 2022-2024
// File: src/routes/blog/mod.rs
// Purpose: Blog module
// Created: March 1, 2024
// Modified: March 13, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use crate::{SiteCfg, mkcontext};

pub async fn blog_index(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(sitecfg.get_ref().to_owned(), "blog", "root").unwrap();
    
    let s = match tmpl.render("blog_menu.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn blog_post(page: web::Path<String>, tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let blogcfg = &sitecfg.blogcfg.pages;

    if !blogcfg.contains_key(&page.clone()) {
        return Ok(HttpResponse::NotFound().body("Blog post not found"))
    }

    let ctx = mkcontext(sitecfg.get_ref().to_owned(), "blog", &page.clone()).unwrap();

    let s = match tmpl.render("blog_post.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err))
    };

    Ok(s)
}