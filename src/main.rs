// ctclsite-rust - CTCL 2022-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: April 23, 2024

use actix_files as fs;
//use actix_web::body::MessageBody;
//use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{
    web, App, HttpServer
};
//use actix_web::{http, middleware, Error, HttpResponse};
//use actix_web_lab::middleware::{from_fn, Next};
use tera::Tera;
use ctclsite::routes::*;
use ctclsite::*;

//async fn redir(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
//    let sitecfg: &SiteCfg = req.app_data().unwrap();
//    let requri = req.uri().clone();
//
//    if sitecfg.globalcfg.redirects.contains_key(&requri.to_string()) {
//        let url = sitecfg.globalcfg.redirects.get(&requri.to_string());
//
//        return Ok(req.into_response(
//            HttpResponse::MovedPermanently()
//                .append_header((http::header::LOCATION, url))
//                .finish(),
//        ));
//    }


//    next.call(req).await
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let globalcfg: GlobalCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();
    let bindip = globalcfg.bindip;
    let bindport = globalcfg.bindport;
    
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();

        let globalcfg: GlobalCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();
        let sitecfg: SiteCfg = SiteCfg {
            themes: globalcfg.clone().themes,
            aboutcfg: serde_json::from_str(&read_file(globalcfg.pages.get("about").unwrap()).unwrap()).unwrap(), 
            linklistcfg: serde_json::from_str(&read_file(globalcfg.pages.get("linklist").unwrap()).unwrap()).unwrap(), 
            blogcfg: serde_json::from_str(&read_file(globalcfg.pages.get("blog").unwrap()).unwrap()).unwrap(), 
            projectscfg: serde_json::from_str(&read_file(globalcfg.pages.get("projects").unwrap()).unwrap()).unwrap(), 
            servicescfg: serde_json::from_str(&read_file(globalcfg.pages.get("services").unwrap()).unwrap()).unwrap(),
            globalcfg: globalcfg.clone()
        };

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(sitecfg))
            // Temporary redirects until a proper middleware is implemented
            .service(web::redirect("/projects/nonmonolithic/", "/projects/nonmono/"))
            .service(web::redirect("/bcc_cc/", "/links/bcc_tc/"))
            .service(web::redirect("/bcc_tc/", "/links/bcc_tc/"))
            .service(web::redirect("/projects/pc_pbt/", "/projects/wbpc/#pc_pbt"))
            .service(web::redirect("/projects/pc_pc/", "/projects/wbpc/#pc_pc"))
            .service(web::redirect("/projects/pc_pet/", "/projects/wbpc/#pc_pet"))
            .service(web::redirect("/projects/pc_pet2/", "/projects/wbpc/#pc_pet2"))
            .service(web::redirect("/projects/pc_pp/", "/projects/wbpc/#pc_pp"))
            .service(web::redirect("/projects/pc_pe/", "/projects/wbpc/#pc_pe"))
            .service(web::redirect("/projects/srv_amp/", "/projects/svcs/"))
            .service(web::redirect("/projects/pc_tbm/", "/projects/misc/#pc_tbm"))
            .service(web::redirect("/projects/pc_r12/", "/projects/misc/#pc_r12"))
            .service(web::redirect("/projects/pc_thc/", "/projects/misc/#pc_thc"))
            .service(web::redirect("/projects/srv_vc/", "/projects/misc/#srv_vc"))
            .service(web::resource("/").route(web::get().to(about_index)))
            .service(web::resource("/privacy/").route(web::get().to(about_privacy)))
            .service(web::resource("/licensing/").route(web::get().to(about_licensing)))
            .service(web::resource("/services/").route(web::get().to(services_index)))
            .service(web::resource("/blog/").route(web::get().to(blog_index)))
            .service(web::resource("/blog/{page}/").route(web::get().to(blog_post)))
            .service(web::resource("/projects/").route(web::get().to(projects_index)))
            .service(web::resource("/projects/{page}/").route(web::get().to(projects_page)))
            .service(web::resource("/links/").route(web::get().to(linklist)))
            .service(web::resource("/inlog/").route(web::post().to(logger_incoming)))
    })
    .bind((bindip, bindport))?
    .run()
    .await
}
