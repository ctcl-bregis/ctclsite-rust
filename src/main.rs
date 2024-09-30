// ctclsite-rust - CTCL 2020-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: September 29, 2024

//use std::collections::HashMap;

use std::thread::available_parallelism;

use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::body::MessageBody;
use actix_web::{http, web, App, Error, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web_lab::middleware::{from_fn, Next};
use ctclsite::{loadconfig, logger::logaccess, loggersetup, read_file, PartialSiteConfig, SiteConfig};
use indexmap::IndexMap;
use memcache::Client;
use minify_html::{minify, Cfg};
use lysine::{Context, Lysine};

use log::{debug, info, warn, LevelFilter, SetLoggerError};
use log::{Record, Level, Metadata};

//use uuid::Uuid;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
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
        .map(|()| log::set_max_level(LevelFilter::Debug))
}

async fn middleware(req: ServiceRequest, next: Next<actix_web::body::BoxBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let sitecfg = req.app_data::<Data<SiteConfig>>().unwrap();
    let requri = req.uri().clone().to_string();

    if requri == "/robots.txt" {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).body(sitecfg.robots.clone()),
        ));
    }

    // No need to check if the URI is "/robots.txt" as such is tested above
    if !requri.ends_with('/') && !requri.starts_with("/static/") {
        let url = format!("{}/", requri);
        return Ok(req.into_response(
            HttpResponse::MovedPermanently()
                .append_header((http::header::LOCATION, url))
                .finish(),
        ));
    }

    if sitecfg.redirects.contains_key(&requri) {
        let url = sitecfg.redirects.get(&requri).unwrap().clone();

        return Ok(req.into_response(
            HttpResponse::MovedPermanently()
                .append_header((http::header::LOCATION, url))
                .finish(),
        ));
    }

    next.call(req).await
}

//pub async fn incominglog(logdata: web::Json<ClientLogEntry>, memclient: web::Data<Option<Client>>) {

//}


pub async fn routepage(req: HttpRequest, page: web::Path<String>, tmpl: web::Data<lysine::Lysine>, sitecfg: web::Data<SiteConfig>, memclient: web::Data<Option<Client>>) -> Result<impl Responder, Error> {
    let mut ctx = Context::new();

    let pagecfg = match page.as_str() {
        "" => sitecfg.pages.get("/").unwrap(),
        _ => match sitecfg.pages.get(page.as_str()) {
            Some(p) => p,
            None => return Ok(HttpResponse::NotFound().body(format!("Page {} not found", page)))
        }
    };

    match memclient.get_ref() {
        Some(m) => {
            logaccess(&sitecfg, req, m)?;
        },
        None => ()
    };

    let theme = match sitecfg.themes.get(&pagecfg.theme) {
        Some(t) => t,
        None => sitecfg.themes.get(&sitecfg.defaulttheme).unwrap()
    };

    ctx.insert("sitedomain", &sitecfg.sitedomain);
    if pagecfg.shownavbar {
        ctx.insert("navbar", &sitecfg.navbar);   
    }

    ctx.insert("title", &pagecfg.title);
    ctx.insert("theme", theme);
    ctx.insert("desc", &pagecfg.desc);
    ctx.insert("keywords", &pagecfg.keywords);
    ctx.insert("favicon", &pagecfg.favicon);
    ctx.insert("headerids", &pagecfg.headerids);
    ctx.insert("rendered", &pagecfg.content);

    match &sitecfg.uservars {
        Some(p) => {
            for (key, value) in p.iter() {
                ctx.insert(key, value);
            }
        },
        None => ()
    }

    // Page-specific variables have a higher precedence over site variables
    match &pagecfg.uservars {
        Some(p) => {
            for (key, value) in p.iter() {
                ctx.insert(key, value);
            }
        },
        None => ()
    }

    // dir name, absolute path
    let mut pathmap: IndexMap<String, String> = IndexMap::new();
    let mut absolute = String::from("/");
    for splitpath in page.split("/") {
        if !splitpath.is_empty() {
            absolute.push_str(&format!("{splitpath}/"));
            pathmap.insert(splitpath.to_owned(), absolute.clone());
        }
    }
    ctx.insert("path", &pathmap);

    let html = match tmpl.render("page.html", &ctx) {
        Ok(html) => html,
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to render template: {:?}", err)))
    };

    let htmlbytes = html.as_bytes();

    let cfg = Cfg {
        do_not_minify_doctype: true,
        ensure_spec_compliant_unquoted_attribute_values: true,
        keep_closing_tags: true,
        keep_html_and_head_opening_tags: true,
        keep_spaces_between_attributes: true,
        keep_comments: false,
        keep_input_type_text_attr: true,
        keep_ssi_comments: false,
        // If any template-related syntax is still in the output, something is quite wrong. Keep for debugging purposes.
        preserve_brace_template_syntax: true,
        preserve_chevron_percent_template_syntax: true,
        // Rendered HTML should have neither CSS or JS. If they do, it is most likely for debugging purposes.
        minify_css: false,
        minify_js: false,
        remove_bangs: false,
        remove_processing_instructions: true
    };

    let htmlmin = match sitecfg.minimizehtml {
        true => minify(htmlbytes, &cfg),
        false => htmlbytes.to_vec(),
    };

    Ok(HttpResponse::Ok().body(htmlmin))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = init();
    
    let partialsiteconfig: PartialSiteConfig = serde_json::from_str(&read_file("ctclsite-config/config.json").unwrap()).unwrap();
    let bindip = partialsiteconfig.bindip;
    let bindport = partialsiteconfig.bindport;

    // Actix-Web already uses available_parallelism() to determine CPU thread count so using this function should be the same as using the default CPU count of Actix-Web
    let syscpus = available_parallelism().unwrap().get();

    // Let 0 default to system CPU count
    let cpus = if partialsiteconfig.cpus == 0 {
        syscpus
    } else if partialsiteconfig.cpus > syscpus {
        warn!("CPU count greater than available system CPU count. Using system CPU count.");
        syscpus
    } else {
        partialsiteconfig.cpus
    };

    HttpServer::new(|| {
        let lysine = Lysine::new("ctclsite-config/templates/**/*.html").unwrap();
        let sitecfg: SiteConfig = loadconfig().unwrap();

        debug!("Pages Loaded:");
        for (pid, page) in &sitecfg.pages {
            debug!("{pid}");
            for cid in page.content.keys() {
                debug!("    {cid}")
            }
        }
        let pcount = sitecfg.pages.len();
        let tcount = sitecfg.themes.len();
        let fcount = sitecfg.fonts.len();
        info!("Pages loaded: {pcount}");
        info!("Themes loaded: {tcount}");
        info!("Fonts loaded: {fcount}");

        if sitecfg.log.enable {
            let _ = loggersetup(&sitecfg);
        }

        let memclient = match sitecfg.log.enable {
            true => match Client::connect(&*sitecfg.log.memcache) {
                Ok(m) => Some(m),
                Err(e) => {
                    warn!("Error connecting to memcache: {e}");
                    None
                }
            },
            false => None
        };

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(lysine))
            .app_data(web::Data::new(sitecfg))
            .app_data(web::Data::new(memclient))
            .wrap(from_fn(middleware))
            .service(web::resource("/{page:.*}").route(web::get().to(routepage)))
    })
    .bind((bindip, bindport))?
    .workers(cpus)
    .run()
    .await
}
