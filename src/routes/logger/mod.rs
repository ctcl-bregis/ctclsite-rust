// ctclsite-rust - CTCL 2022-2024
// File: src/logger/mod.rs
// Purpose: Logger routes
// Created: March 3, 2024
// Modified: March 14, 2024

use std::io::Write;
use std::fs::OpenOptions;
use actix_web::{
    web, Error, HttpResponse, HttpRequest, Responder, Result,
};
use serde::{Deserialize, Serialize};
use csv;

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientLog {
    time_zone: String,
    ext_ip: String,
    // Device data
    web_gl_vendor: String,
    web_gl_renderer: String,
    cpu_cores: String,
    mem_size: String,
    max_tp:  String,
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

pub async fn logger_incoming(req: HttpRequest, mut info: web::Json<ClientLog>) -> Result<impl Responder, Error> {
    if !std::path::Path::new("logs/").exists() {
        std::fs::create_dir("logs/")?;
    }

    if !std::path::Path::new("logs/client_latest.csv").exists() {
        let mut file = std::fs::File::create("logs/client_latest.csv")?;
        file.write_all(b"time_zone,ext_ip,web_gl_vendor,web_gl_renderer,cpu_cores,mem_size,max_tp,oscpu,plat,screen_x,screen_y,screen_pix_ratio,screen_pix_depth,canvas_fp,online,pdf_viewer,cookies_enabled,dnt_enabled,langs,prod,prod_sub,user_agent,vend,inner_height,inner_width\n")?;
    }

    let conninfo = &req.connection_info();
    let ip = conninfo.realip_remote_addr().unwrap();

    info.ext_ip = ip.to_string();

    let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(vec![]);
    writer.serialize(info).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("logs/client_latest.csv")
        .unwrap();

    let mut data = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    // Strip newline
    data.pop();

    writeln!(file, "{}", data).unwrap();

    Ok(HttpResponse::Ok().take())
}