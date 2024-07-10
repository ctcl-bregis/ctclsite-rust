// ctclsite-rust - CTCL 2020-2024
// File: src/build.rs
// Purpose: Build needed files
// Created: February 28, 2024
// Modified: July 10, 2024

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::path::Path;
use std::result::Result;
use tera::{Tera, Context};
use minifier::js::minify;
extern crate image;
use image::{Rgb, RgbImage};

#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
    // Main theme color
    color: String,
    // Text color on theme color
    fgcolor: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StylingCfg {
    vars: HashMap<String, String>,
    fonts: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SiteCfg {
    pub themes: HashMap<String, Theme>,
    pub styling: StylingCfg,
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();

    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn mkdir(path: &str) -> Result<(), Error> {
    // Do not return an error if it exists, just continue
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir(path).expect("Could not create directory static/favicons/");
    }
    Ok(())
}

fn main() {
    let sitecfg: SiteCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();

    // Step 1: Build CSS
    let tera = Tera::new("src/styling/**/*.css").unwrap();
    let mut ctx = Context::new();

    mkdir("static/themes").unwrap();

    // Step 1.1: Build base.css
    for (name, value) in sitecfg.styling.vars.iter() {
        ctx.insert(name, value);
    }
    ctx.insert("fonts", &sitecfg.styling.fonts);
    ctx.insert("themes", &sitecfg.themes);

    let base = tera.render("base.css", &ctx).unwrap();
    
    std::fs::write("static/base.css", base).unwrap();

    // Step 1.2: Build themes
    for (key, value) in sitecfg.themes.clone().iter() {
        let mut ctx = Context::new();
        ctx.insert("themecolor", &value.color);
        ctx.insert("fgcolor", &value.fgcolor);
        let theme = tera.render("theme.css", &ctx).unwrap();

        std::fs::write(&format!("static/themes/{}.css", key), theme).unwrap();
    }

    // Step 2: Minimize and move JavaScript
    let paths = std::fs::read_dir("src/js/").unwrap();

    for path in paths {
        let upath = path.unwrap().file_name();
        let filename = upath.to_str().unwrap();
        let jsfile: String;
        if filename.ends_with(".js") {
            jsfile = read_file(&format!("src/js/{}", filename)).unwrap_or_else(|_| panic!("Error reading src/js/{}", filename));
            let minjsfile = minify(&jsfile).to_string();
            std::fs::write(&format!("static/{}", filename), minjsfile).unwrap();
        }
    }

    // Step 3: Generate default favicons
    mkdir("static/favicons/").unwrap();

    for (key, value) in sitecfg.themes.clone().iter() {
        // It is unlikely that default favicons would change so to reduce build time and disk writes, generation is skipped if the favicon already exists.
        if !file_exists("static/favicons/default_{key}.ico") {
            let mut image = RgbImage::new(16, 16);
            for x in 0..16 {
                for y in 0..16 {
                    let mut bytes = [0u8; 3];
                    hex::decode_to_slice(&value.color.replace('#', ""), &mut bytes as &mut [u8]).unwrap();
                    image.put_pixel(x, y, Rgb(bytes));
                }
            }
            image.save(format!("static/favicons/default_{key}.ico")).unwrap_or_else(|_| panic!("Error while saving file default_{key}.ico"))
        }
    }

}