// ctclsite-rust - CTCL 2022-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: March 17, 2024

use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpServer
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tera::Tera;
use ctclsite::*;
use ctclsite::routes::*;

// config/config.json
#[derive(Deserialize, Serialize, Clone)]
pub struct GlobalCfg {
    pages: HashMap<String, String>,
    themes: HashMap<String, Theme>,
    bindip: String,
    bindport: u16
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let globalcfg: GlobalCfg = serde_json::from_str(&read_file("config/config.json".to_string()).unwrap()).unwrap();
    let bindip = globalcfg.bindip;
    let bindport = globalcfg.bindport;
    
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();

        let globalcfg: GlobalCfg = serde_json::from_str(&read_file("config/config.json".to_string()).unwrap()).unwrap();
        let sitecfg: SiteCfg = SiteCfg {
            themes: globalcfg.themes,
            themes_css: serde_json::from_str(&read_file("themes.json".to_string()).unwrap()).unwrap(),
            aboutcfg: serde_json::from_str(&read_file(globalcfg.pages.get("about").unwrap().to_string()).unwrap()).unwrap(), 
            bcctccfg: serde_json::from_str(&read_file(globalcfg.pages.get("bcctc").unwrap().to_string()).unwrap()).unwrap(), 
            blogcfg: serde_json::from_str(&read_file(globalcfg.pages.get("blog").unwrap().to_string()).unwrap()).unwrap(), 
            projectscfg: serde_json::from_str(&read_file(globalcfg.pages.get("projects").unwrap().to_string()).unwrap()).unwrap(), 
            servicescfg: serde_json::from_str(&read_file(globalcfg.pages.get("services").unwrap().to_string()).unwrap()).unwrap()
        };

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(sitecfg))
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Always))
            .service(web::resource("/").route(web::get().to(about_index)))
            .service(web::resource("/privacy/").route(web::get().to(about_privacy)))
            .service(web::resource("/licensing/").route(web::get().to(about_licensing)))
            .service(web::resource("/services/").route(web::get().to(services_index)))
            .service(web::resource("/blog/").route(web::get().to(blog_index)))
            .service(web::resource("/blog/{page}/").route(web::get().to(blog_post)))
            .service(web::resource("/projects/").route(web::get().to(projects_index)))
            .service(web::resource("/projects/{page}/").route(web::get().to(projects_page)))
            .service(web::redirect("/projects/nonmonolithic", "/projects/nonmono")) // Quick hack that should be removed in a future update
            .service(web::resource("/bcc_tc/").route(web::get().to(bcctc_index)))
            .service(web::redirect("/bcc_cc/", "/bcc_tc/"))
            .service(web::resource("/inlog/").route(web::post().to(logger_incoming)))
    })
    .bind((bindip, bindport))?
    .run()
    .await
}
