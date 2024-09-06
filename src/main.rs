// ctclsite-rust - CTCL 2020-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: September 5, 2024

use actix_files as fs;
use actix_web::web::Data;
use actix_web::body::MessageBody;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web_lab::middleware::{from_fn, Next};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let combinedcfg: CombinedCfg = get_combined_cfg().expect("config load failed");
    //let bindip = combinedcfg.sitecfg.bindip;
    //let bindport = combinedcfg.sitecfg.bindport;
    
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();

        //let combinedcfg: CombinedCfg = get_combined_cfg().expect("config load failed");

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            //.app_data(web::Data::new(combinedcfg))
            //.wrap(from_fn(redir))
            .service(web::resource("/{page:.*}").route(web::get().to(routepage)))
    })
    .bind((bindip, bindport))?
    .run()
    .await
}