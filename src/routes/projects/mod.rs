// ctclsite-rust - CTCL 2020-2024
// File: src/routes/projects/mod.rs
// Purpose: Projects module
// Created: March 1, 2024
// Modified: August 4, 2024

use actix_web::{web, Error, HttpResponse, Responder, Result};
use crate::{CombinedCfg, mkcontext};

pub async fn projects_index(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(&combinedcfg, "projects", combinedcfg.projects.get("root").unwrap()).unwrap();
    
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
            "docs" => "docs.html",
            _ => return Ok(HttpResponse::InternalServerError().body("Invalid page type"))
        }
        None => return Ok(HttpResponse::NotFound().body(format!("Page {} not found", page)))
    };

    let ctx = match mkcontext(&combinedcfg, "projects", combinedcfg.projects.get(page.as_ref()).unwrap()) {
        Ok(ctx) => ctx,
        Err(err) => match err.kind() {
            std::io::ErrorKind::InvalidInput => return Ok(HttpResponse::NotFound().body("Page not found")),
            _ => return Ok(HttpResponse::InternalServerError().body(format!("Page load failed: {:?}", err)))
        }
    };
    
    let s = match tmpl.render(template, &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Template rendering failed: {:?}", err)))
    };

    Ok(s)
}

pub async fn projects_subpage(page: web::Path<(String, String)>, tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>) -> Result<impl Responder, Error> {

    let pagecfg = match combinedcfg.projects.get(&page.0) {
        None => return Ok(HttpResponse::NotFound().body(format!("Page \"{}\" not found in projects", &page.0))),
        Some(projectpage) => match projectpage.pages.as_ref() {
            None => return Ok(HttpResponse::NotFound().body(format!("Page {} does not have subpages", &page.0))),
            Some(subpages) => match subpages.get(&page.1) {
                None => return Ok(HttpResponse::NotFound().body(format!("Subpage {} not found in {}", &page.0, &page.1))),
                Some(subpage) => subpage
            }
        }
    };

    let ctx = match mkcontext(&combinedcfg, "projects", pagecfg) {
        Ok(ctx) => ctx,
        Err(err) => match err.kind() {
            std::io::ErrorKind::InvalidInput => return Ok(HttpResponse::NotFound().body("Page not found")),
            _ => return Ok(HttpResponse::InternalServerError().body(format!("Page load failed: {:?}", err)))
        }
    };
    
    let s = match tmpl.render("content.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Template rendering failed: {:?}", err)))
    };

    Ok(s)
}