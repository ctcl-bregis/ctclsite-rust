// ctclsite-rust - CTCL 2022-2024
// File: src/logger/mod.rs
// Purpose: Logger routes
// Created: March 3, 2024
// Modified: March 15, 2024

use std::io::Write;
use std::fs::OpenOptions;
use chrono::prelude::*;
use actix_web::{
    web, Error, HttpResponse, HttpRequest, Responder, Result,
};
use serde::{Deserialize, Serialize};
use csv;

// For use with data received from the client
#[derive(Deserialize, Serialize, Debug)]
pub struct IncomingClientLog {
    url_path: String,
    // Device data
    time_zone: String,
    web_gl_vendor: String,
    web_gl_renderer: String,
    cpu_cores: String,
    mem_size: String,
    max_tp: String,
    oscpu: String,
    plat: String,
    screen_x: String,
    screen_y: String,
    screen_pix_ratio: String,
    screen_pix_depth: String,
    canvas_fp: String,
    // Software support
    online: String,
    pdf_viewer: String,
    cookies_enabled: String,
    dnt_enabled: String,
    langs: String,
    prod: String,
    prod_sub: String,
    user_agent: String,
    vend: String,
    inner_height: String,
    inner_width: String,
}

// Client Log entry with fields added server-side
#[derive(Deserialize, Serialize, Debug)]
pub struct ClientLog {
    timestamp: String, // Added server-side
    ext_ip: String, // Added server-side
    url_path: String,
    // Device data
    time_zone: String,
    web_gl_vendor: String,
    web_gl_renderer: String,
    cpu_cores: String,
    mem_size: String,
    max_tp: String,
    oscpu: String,
    plat: String,
    screen_x: String,
    screen_y: String,
    screen_pix_ratio: String,
    screen_pix_depth: String,
    canvas_fp: String,
    // Software support
    online: String,
    pdf_viewer: String,
    cookies_enabled: String,
    dnt_enabled: String,
    langs: String,
    prod: String,
    prod_sub: String,
    user_agent: String,
    vend: String,
    inner_height: String,
    inner_width: String,
}

fn icl2cl(icl: IncomingClientLog, extip: &str) -> ClientLog {
    let ts = Utc::now().to_rfc3339().to_string();
    ClientLog {
        timestamp: ts,
        ext_ip: extip.to_owned(),
        url_path: icl.url_path,
        time_zone: icl.time_zone,
        web_gl_vendor: icl.web_gl_vendor,
        web_gl_renderer: icl.web_gl_renderer,
        cpu_cores: icl.cpu_cores,
        mem_size: icl.mem_size,
        max_tp: icl.max_tp,
        oscpu: icl.oscpu,
        plat: icl.plat,
        screen_x: icl.screen_x,
        screen_y: icl.screen_y,
        screen_pix_ratio: icl.screen_pix_ratio,
        screen_pix_depth: icl.screen_pix_depth,
        canvas_fp: icl.canvas_fp,
        online: icl.online,
        pdf_viewer: icl.pdf_viewer,
        cookies_enabled: icl.cookies_enabled,
        dnt_enabled: icl.dnt_enabled,
        langs: icl.langs,
        prod: icl.prod,
        prod_sub: icl.prod_sub,
        user_agent: icl.user_agent,
        vend: icl.vend,
        inner_height: icl.inner_height,
        inner_width: icl.inner_width
    }
}


pub async fn logger_incoming(req: HttpRequest, info: web::Json<IncomingClientLog>) -> Result<impl Responder, Error> {
    // The logs directory is excluded altogether with .gitignore and ./mkrelease
    if !std::path::Path::new("logs/").exists() {
        std::fs::create_dir("logs/")?;
    }

    // Manually write the CSV header
    if !std::path::Path::new("logs/client_latest.csv").exists() {
        let mut file = std::fs::File::create("logs/client_latest.csv")?;
        file.write_all(b"timestamp,ext_ip,url_path,time_zone,web_gl_vendor,web_gl_renderer,cpu_cores,mem_size,max_tp,oscpu,plat,screen_x,screen_y,screen_pix_ratio,screen_pix_depth,canvas_fp,online,pdf_viewer,cookies_enabled,dnt_enabled,langs,prod,prod_sub,user_agent,vend,inner_height,inner_width\n")?;
    }

    let conn = &req.connection_info();
    let extip = conn.realip_remote_addr().unwrap();
    let info = icl2cl(info.0, extip);

    let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(vec![]);
    writer.serialize(info).unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .open("logs/client_latest.csv")
        .unwrap();

    let mut data = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    // Strip newline
    data.pop();

    writeln!(file, "{}", data).unwrap();

    Ok(HttpResponse::Ok().take())
}