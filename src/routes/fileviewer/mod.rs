// ctclsite-rust - CTCL 2020-2024
// File: src/routes/fileviewer/mod.rs
// Purpose: Module for the File Viewer feature
// Created: August 2, 2024
// Modified: August 2, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result
};
use std::fs;
use tera::Context;
use crate::{CombinedCfg, read_file};

pub async fn fileviewer(tmpl: web::Data<tera::Tera>, combinedcfg: web::Data<CombinedCfg>, page: web::Path<(String, String)>) -> Result<impl Responder, Error> {
    let mut ctx = Context::new();
    let staticpath = format!("static/{}.{}", page.0, page.1);

    let filetype = match combinedcfg.sitecfg.fileviewer.extensions.get(&page.1) {
        Some(typ) => typ,
        None => "binary"
    };
    ctx.insert("filetype", filetype);
    ctx.insert("filepath", &staticpath);
    ctx.insert("filesize", &fs::metadata(&staticpath)?.len());
    ctx.insert("title", &format!("File: {}", &staticpath));
    ctx.insert("themename", "gold");

    ctx.insert("titlesizeoverride", "16pt");

    if filetype == "plaintext" {
        ctx.insert("content", &read_file(&staticpath).unwrap());
    }

    let s = match tmpl.render("fileviewer.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err))
    };

    Ok(s)
}