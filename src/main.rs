// ctclsite-rust - CTCL 2022-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: March 4, 2024

//use std::collections::HashMap;

use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpServer
};
use tera::Tera;
use ctclsite::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(about_index)))
            .service(web::resource("/privacy/").route(web::get().to(about_privacy)))
            .service(web::resource("/licensing/").route(web::get().to(about_licensing)))
            .service(web::resource("/services/").route(web::get().to(services_index)))
            .service(web::resource("/blog/").route(web::get().to(blog_index)))
            .service(web::resource("/blog/{page}/").route(web::get().to(blog_post)))
            .service(web::resource("/projects/").route(web::get().to(projects_index)))
            .service(web::resource("/projects/{page}/").route(web::get().to(projects_page)))
            .service(web::resource("/inlog/").route(web::post().to(logger_incoming)))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
