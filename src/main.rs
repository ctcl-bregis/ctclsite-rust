// ctclsite-rust - CTCL 2020-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: September 8, 2024

use std::borrow::Borrow;

use actix_files as fs;
use actix_web::web::Data;
use actix_web::body::MessageBody;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web_lab::middleware::{from_fn, Next};
use ctclsite::{loadconfig, loadpages, read_file, PartialSiteConfig, SiteConfig};
use memcache::Client;
use tera::{Context, Tera};

use log::{warn, LevelFilter, SetLoggerError};
use log::{Record, Level, Metadata};

use ctclsite::*;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}

pub async fn routepage(page: web::Path<String>, tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteConfig>) -> Result<impl Responder, Error> {
    let mut ctx = Context::new();

    dbg!(&sitecfg.pages);

    let pagecfg = match sitecfg.pages.get(page.as_str()) {
        Some(p) => p,
        None => return Ok(HttpResponse::NotFound().body(format!("Page {} not found", page)))
    };

    let html = match pagecfg {
        Page::Content(p) => {
            ctx.insert("title", &p.title);
            ctx.insert("theme", &p.theme);
            ctx.insert("desc", &p.title);
            ctx.insert("keywords", &p.title);
            ctx.insert("favicon", &p.title);
            ctx.insert("content", &read_file(&p.content).unwrap());

            match tmpl.render("content.html", &ctx) {
                Ok(html) => html,
                Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
            }
        },
        Page::Linklist(p) => {
            ctx.insert("title", &p.title);
            ctx.insert("theme", &p.theme);
            ctx.insert("desc", &p.title);
            ctx.insert("keywords", &p.title);
            ctx.insert("favicon", &p.title);

            match tmpl.render("linklist.html", &ctx) {
                Ok(html) => html,
                Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
            }

        },
        Page::Sections(p) => {
            ctx.insert("title", &p.title);
            ctx.insert("theme", &p.theme);
            ctx.insert("desc", &p.title);
            ctx.insert("keywords", &p.title);
            ctx.insert("favicon", &p.title);
            ctx.insert("sections", &loadsections(&p.sections));

            match tmpl.render("sections.html", &ctx) {
                Ok(html) => html,
                Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render the template: {:?}", err)))
            }
        }
    };


    Ok(HttpResponse::Ok().body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = init();
    
    let partialsiteconfig: PartialSiteConfig = serde_json::from_str(&read_file("ctclsite-config/config.json").unwrap()).unwrap();
    let bindip = partialsiteconfig.bindip;
    let bindport = partialsiteconfig.bindport;
    let cpus = partialsiteconfig.cpus;


    HttpServer::new(|| {
        let tera = Tera::new("ctclsite-config/templates/**/*.html").unwrap();
        let siteconfig: SiteConfig = loadconfig().unwrap();

        let memclient: Option<Client> = match memcache::connect("memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true") {
            Ok(m) => {
                Some(m)
            },
            Err(e) => match e {
                memcache::MemcacheError::BadURL(me) => {
                    warn!("memcache connection error - invalid URL: {me}");
                    None
                },
                memcache::MemcacheError::IOError(_) => {
                    warn!("");
                    None
                },
                memcache::MemcacheError::ClientError(_) => {
                    warn!("");
                    None
                },
                memcache::MemcacheError::ServerError(_) => {
                    warn!("");
                    None
                },
                memcache::MemcacheError::CommandError(_) => {
                    warn!("");
                    None
                },
                memcache::MemcacheError::OpensslError(_) => {
                    warn!("");
                    None
                },
                memcache::MemcacheError::ParseError(_) => {
                    warn!("");
                    None
                },
                memcache::MemcacheError::PoolError(_) => {
                    warn!("");
                    None
                }, 
            }
        };

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(siteconfig))
            .app_data(web::Data::new(memclient))
            //.wrap(from_fn(redir))
            .service(web::resource("/{page:.*}").route(web::get().to(routepage)))
    })
    .bind((bindip, bindport))?
    .workers(cpus)
    .run()
    .await
}