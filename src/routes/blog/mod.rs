// ctclsite-rust - CTCL 2020-2024
// File: src/routes/blog/mod.rs
// Purpose: Blog module
// Created: March 1, 2024
// Modified: May 29, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use tera::Context;
use crate::{mdpath2html, SiteCfg};

fn mkcontext(sitecfg: &SiteCfg, subpage: &str) -> Context {
    let mut ctx = Context::new();

    if subpage == "root" {
        let subpagetype = &sitecfg.blogcfg.root;
        ctx.insert("posts", &sitecfg.blogcfg.posts);
        ctx.insert("title", &subpagetype.title);
        let defaultfavicon = &format!("/static/favicons/default_{}.ico", &subpagetype.theme);
        ctx.insert("favicon", match &subpagetype.favicon {
            Some(link) => link,
            None => defaultfavicon
        });
        ctx.insert("themename", &subpagetype.theme);
        ctx.insert("themecolor", &sitecfg.themes.get(&subpagetype.theme).unwrap().color);
        ctx.insert("desc", &subpagetype.desc);
        ctx.insert("keywords", &subpagetype.keywords);
    } else {
        // The page should be known to exist at this point
        let subpagetype = sitecfg.blogcfg.posts.get(subpage).unwrap();

        ctx.insert("title", &subpagetype.title);
        let defaultfavicon = &format!("/static/favicons/default_{}.ico", &subpagetype.theme);
        ctx.insert("favicon", match &subpagetype.favicon {
            Some(link) => link,
            None => defaultfavicon
        });
        ctx.insert("themename", &subpagetype.theme);
        ctx.insert("themecolor", &sitecfg.themes.get(&subpagetype.theme).unwrap().color);
        ctx.insert("desc", &subpagetype.desc);
        ctx.insert("keywords", &subpagetype.keywords);

        ctx.insert("rendered", &mdpath2html(&subpagetype.content, true).unwrap());        
    }
    ctx
}

pub async fn blog_index(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(sitecfg.get_ref(), "root");
    
    let s = match tmpl.render("blog_menu.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
    };

    Ok(s)
}

pub async fn blog_post(page: web::Path<String>, tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let blogcfg = &sitecfg.blogcfg;

    if !blogcfg.posts.contains_key(&page.clone()) {
        return Ok(HttpResponse::NotFound().body("Blog post not found"))
    }

    let ctx = mkcontext(sitecfg.get_ref(), &page.clone());

    let s = match tmpl.render("content.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err))
    };

    Ok(s)
}