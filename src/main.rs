// ctclsite-rust - CTCL 2020-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: July 28, 2024

use actix_files as fs;
use actix_web::web::Data;
use actix_web::body::MessageBody;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web_lab::middleware::{from_fn, Next};
use tera::Tera;
use ctclsite::routes::*;
use ctclsite::*;

async fn redir(req: ServiceRequest, next: Next<actix_web::body::BoxBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let combinedcfg = req.app_data::<Data<CombinedCfg>>().unwrap();
    let requri = req.uri().clone().to_string();

    if !requri.ends_with('/') && requri != "/robots.txt" {
        let url = format!("{}/", requri);
        return Ok(req.into_response(
            HttpResponse::MovedPermanently()
                .append_header((http::header::LOCATION, url))
                .finish(),
        ));
    }

    if combinedcfg.sitecfg.redirects.contains_key(&requri) {
        let url = combinedcfg.sitecfg.redirects.get(&requri).unwrap().clone();

        return Ok(req.into_response(
            HttpResponse::MovedPermanently()
                .append_header((http::header::LOCATION, url))
                .finish(),
        ));
    }

    next.call(req).await
}

pub async fn robots() -> Result<impl Responder, Error> {
    let robotstxt = read_file("static/robots.txt").unwrap();

    Ok(robotstxt)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let combinedcfg: CombinedCfg = get_combined_cfg().expect("config load failed");
    let bindip = combinedcfg.sitecfg.bindip;
    let bindport = combinedcfg.sitecfg.bindport;
    
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();

        let combinedcfg: CombinedCfg = get_combined_cfg().expect("config load failed");

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(combinedcfg))
            .wrap(from_fn(redir))
            .service(web::resource("/robots.txt").route(web::get().to(robots)))
            .service(web::resource("/").route(web::get().to(about_index)))
            .service(web::resource("/privacy/").route(web::get().to(about_privacy)))
            .service(web::resource("/licensing/").route(web::get().to(about_licensing)))
            .service(web::resource("/blog/").route(web::get().to(blog_index)))
            .service(web::resource("/blog/{page}/").route(web::get().to(blog_post)))
            .service(web::resource("/inlog/").route(web::post().to(logger_incoming)))
            .service(web::resource("/links/{page}/").route(web::get().to(linklist)))
            .service(web::resource("/projects/").route(web::get().to(projects_index)))
            .service(web::resource("/projects/{page}/").route(web::get().to(projects_page)))
            .service(web::resource("/projects/{page}/{subpage}/").route(web::get().to(projects_subpage)))
            .service(web::resource("/services/").route(web::get().to(services_index)))
    })
    .bind((bindip, bindport))?
    .run()
    .await
}
