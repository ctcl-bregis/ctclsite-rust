// ctclsite-rust - CTCL 2020-2024
// File: src/main.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: September 19, 2024

use actix_files as fs;
use actix_web::web::Data;
use actix_web::body::MessageBody;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web_lab::middleware::{from_fn, Next};
use ctclsite::{loadconfig, read_file, PartialSiteConfig, SiteConfig};
use memcache::Client;
use minify_html::{minify, Cfg};
use tera::{Context, Tera};

use log::{debug, info, warn, LevelFilter, SetLoggerError};
use log::{Record, Level, Metadata};

use ctclsite::*;
use uuid::Uuid;

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

async fn redir(req: ServiceRequest, next: Next<actix_web::body::BoxBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let sitecfg = req.app_data::<Data<SiteConfig>>().unwrap();
    let requri = req.uri().clone().to_string();

    if !requri.ends_with('/') && requri != "/robots.txt" && !requri.starts_with("/static/") {
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

pub async fn incominglog(logdata: web::Json<ClientLogEntry>, memclient: web::Data<Option<Client>>) {


}

pub async fn routepage(page: web::Path<String>, tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteConfig>, memclient: web::Data<Option<Client>>) -> Result<impl Responder, Error> {
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
            let uuid = Uuid::new_v4().to_string();
            let _ = m.set(&uuid, false, 120);

            ctx.insert("uuid", &uuid);
            debug!("UUID inserted into memcache: {uuid}");
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
    let cpus = partialsiteconfig.cpus;

    HttpServer::new(|| {
        let tera = Tera::new("ctclsite-config/templates/**/*.html").unwrap();
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

        let memclient = match sitecfg.enablememcache {
            true => match Client::connect(&*sitecfg.memcache) {
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
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(sitecfg))
            .app_data(web::Data::new(memclient))
            .wrap(from_fn(redir))
            .service(web::resource("/{page:.*}").route(web::get().to(routepage)))
    })
    .bind((bindip, bindport))?
    .workers(cpus)
    .run()
    .await
}
