// ctclsite-rust - CTCL 2020-2024
// File: src/build.rs
// Purpose: Build needed files
// Created: February 28, 2024
// Modified: July 17, 2024

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

fn empty3u8() -> [u8; 3] {
    [0u8; 3]
}

fn basestring() -> String {
    "base".to_string()
}

#[derive(Serialize, Deserialize, Clone)]
struct Theme {
    // Main theme color
    color: String,
    #[serde(default = "empty3u8")]
    colorrgb: [u8; 3],
    // Text color on theme color
    fgcolor: String,
    #[serde(default = "empty3u8")]
    fgcolorrgb: [u8; 3],
    // Theme-specific styling, "base" by default
    #[serde(default = "basestring")]
    css: String,
    // Font used to represent the theme, should be the name of a font family defined in "fonts" under "styling"
    mainfont: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
enum VarType {
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "number")]
    Number { value: i32 },
    #[serde(rename = "stringmap")]
    StringMap { value: HashMap<String, String> }
}

#[derive(Serialize, Deserialize, Clone)]
struct Font {
    family: String,
    path: String,
    weight: String,
    style: String
}

#[derive(Serialize, Deserialize, Clone)]
struct StylingCfg {
    vars: HashMap<String, VarType>,
    fonts: Vec<Font>,
    css: HashMap<String, String>,
    themes: HashMap<String, Theme>
}

#[derive(Serialize, Deserialize, Clone)]
struct SiteCfg {
    styling: StylingCfg,
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

    mkdir("static/styling/").unwrap();
    mkdir("static/styling/themes/").unwrap();

    // Step 1.1: Build base stylesheets

    // Iterate "vars" and insert each as a variable, this makes things much easier
    for (name, value) in sitecfg.to_owned().styling.vars.iter() {
        // ctx.insert() cannot insert untyped variables? Probably a better solution could be done for this.
        match value {
            VarType::String { value } => ctx.insert(name, value),
            VarType::Number { value } => ctx.insert(name, value),
            VarType::StringMap { value } => ctx.insert(name, value),
        };
    }
    ctx.insert("fonts", &sitecfg.styling.fonts);

    let mut themes: HashMap<String, Theme> = HashMap::new();
    for (name, data) in sitecfg.to_owned().styling.themes {
        let mut fgcolorrgb = [0u8; 3];
        hex::decode_to_slice(&data.fgcolor.replace('#', ""), &mut fgcolorrgb as &mut [u8]).unwrap();
        let mut colorrgb = [0u8; 3];
        hex::decode_to_slice(&data.color.replace('#', ""), &mut colorrgb as &mut [u8]).unwrap();

        let newdata: Theme = Theme {
            color: data.color.clone(),
            colorrgb,
            fgcolor: data.fgcolor.clone(),
            fgcolorrgb,
            css: data.css.clone(),
            mainfont: data.mainfont.clone()
        };

        themes.insert(name.to_string(), newdata);
    }


    ctx.insert("themes", &themes);
    // This amount of "to_owned" is probably going to own RAM. This is just the build script so probably would not be a problem.
    for (name, theme) in themes.iter() {
        // CSS files starting with "_" should only be used by being included by other files
        let cssfile = match sitecfg.styling.css.get(&theme.css) {
            Some(css) => css,
            None => return eprintln!("Invalid css name: {}", theme.css),
        };

        let mut tempctx = ctx.clone();
        tempctx.insert("color", &theme.color);
        tempctx.insert("colorrgb", &theme.colorrgb);
        tempctx.insert("fgcolor", &theme.fgcolor);
        tempctx.insert("fgcolorrgb", &theme.fgcolorrgb);

        let styling = tera.render(cssfile, &tempctx).unwrap();
        std::fs::write(format!("static/styling/{}.css", name), styling).unwrap();
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

    for (key, value) in sitecfg.to_owned().styling.themes.iter() {
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