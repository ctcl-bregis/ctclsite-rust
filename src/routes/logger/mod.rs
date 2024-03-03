// ctclsite-rust - CTCL 2022-2024
// File: src/logger/mod.rs
// Purpose: Logger routes
// Created: March 3, 2024
// Modified: March 3, 2024

use std::io::Write;
use actix_web::{
    web, Error, HttpResponse, HttpRequest, Responder, Result,
};
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use csv;

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientLog {
    timeZone: String,
    localIp: String,
    extIp: String,
    // Device data
    webGlVendor: String,
    webGlRenderer: String,
    cpuCores: String,
    memSize: String,
    maxTp:  String,
    oscpu: String,
    plat: String,
    screenX: String,
    screenY: String,
    screenPixRatio: String,
    screenPixDepth: String,
    canvasFp: String,
    // Software support
    onLine: String,
    pdfViewer: String,
    cookiesEnabled: String,
    dntEnabled: String,
    langs: String,
    prod: String,
    prodSub: String,
    userAgent: String,
    vend: String,
    innerHeight: String,
    innerWidth: String,
}

pub async fn logger_getip(req: HttpRequest) -> Result<impl Responder, Error> {
    let conninfo = &req.connection_info();
    let ip = conninfo.realip_remote_addr().unwrap();

    Ok(HttpResponse::Ok().body(ip.to_string()))
}

pub async fn logger_incoming(req: HttpRequest, mut info: web::Json<ClientLog>) -> Result<impl Responder, Error> {
    let mut writer = csv::WriterBuilder::new().has_headers(false);
    if !std::path::Path::new("logs/").exists() {
        std::fs::create_dir("logs/")?;
    }

    if !std::path::Path::new("logs/client_latest.csv").exists() {
        let mut file = std::fs::File::create("logs/client_latest.csv")?;
        file.write_all(b"timeZone,localIp,extIp,webGlVendor,webGlRenderer,cpuCores,memSize,maxTp,oscpu,plat,screenX,screenY,screenPixRatio,screenPixDepth,canvasFp,onLine,pdfViewer,cookiesEnabled,dntEnabled,langs,prod,prodSub,userAgent,vend,innerHeight,innerWidth\n")?;
    }

    let conninfo = &req.connection_info();
    let ip = conninfo.realip_remote_addr().unwrap();

    info.extIp = ip.to_string();

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