// ctclsite-rust - CTCL 2020-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: February 25, 2024

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;

mod about;
//pub use crate::about;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}