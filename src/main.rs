// ctclsite - CTCL 2019-2024
// File: src/main.rs
// Purpose: Webapp definition
// Created: November 28, 2022
// Modified: November 6, 2024

use std::thread::available_parallelism;

use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::body::MessageBody;
use actix_web::{http, web, App, Error, HttpResponse, HttpResponseBuilder, HttpServer};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web_lab::middleware::{from_fn, Next};
use ctclsite::routepage;
use ctclsite::{loadconfig, loggersetup, read_file, PartialSiteConfig, SiteConfig};
use memcache::Client;
use lysine::Lysine;

use log::{debug, info, warn, LevelFilter, SetLoggerError};
use log::{Record, Level, Metadata};

//use uuid::Uuid;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(siteconfig: &PartialSiteConfig) -> Result<(), SetLoggerError> {
    let levelfilter = match siteconfig.debugloglevel.as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => {
            panic!("Invalid log level for debugloglevel in config.json")
        }
    };

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(levelfilter))
}

async fn middleware(req: ServiceRequest, next: Next<actix_web::body::BoxBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let sitecfg = req.app_data::<Data<SiteConfig>>().unwrap();
    let requri = req.uri().clone().to_string();

    // Quick hack to add robots.txt support
    if requri == "/robots.txt" {
        return Ok(ServiceResponse::new(
            req.request().clone(),
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).body(sitecfg.robots.clone()),
        ));
    }

    // Redirect if the URL does not end with a trailing slash
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    let siteconfigpath = match read_file("config.txt") {
        Ok(t) => t.strip_suffix("\n").unwrap_or(&t).to_owned(),
        Err(e) => return Err(e)
    };

    let siteconfigjson = match read_file(format!("{siteconfigpath}config.json")) {
        Ok(t) => t.strip_suffix("\n").unwrap_or(&t).to_owned(),
        Err(e) => return Err(e)
    };

    let partialsiteconfig: PartialSiteConfig = match serde_json::from_str::<PartialSiteConfig>(&siteconfigjson) {
        Ok(t) => t,
        Err(e) => return Err(e.into()),
    };

    let bindip = &partialsiteconfig.bindip;
    let bindport = partialsiteconfig.bindport;

    let _ = init(&partialsiteconfig);

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
        let sitecfg: SiteConfig = loadconfig().unwrap();
        let siteconfigpath = read_file("config.txt").unwrap();
        let lysine = Lysine::new(&format!("{}/templates/**/*.lish", siteconfigpath)).unwrap();

        debug!("Pages Loaded:");
        for (pid, page) in &sitecfg.pages {
            debug!("{pid}");
            for cid in page.content.keys() {
                debug!("    {cid}")
            }
        }
        
        info!("Pages loaded: {}", sitecfg.pages.len());
        info!("Themes loaded: {}", sitecfg.themes.len());
        info!("Fonts loaded: {}", sitecfg.fonts.len());

        if sitecfg.logger.enable {
            let _ = loggersetup(&sitecfg);
        }

        let memclient = match sitecfg.logger.enable {
            true => match Client::connect(&*sitecfg.logger.memcache) {
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
    .bind((bindip.as_str(), bindport))?
    .workers(cpus)
    .run()
    .await
}
