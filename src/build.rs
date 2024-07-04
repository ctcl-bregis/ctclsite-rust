// ctclsite-rust - CTCL 2020-2024
// File: src/build.rs
// Purpose: Build needed files
// Created: February 28, 2024
// Modified: June 29, 2024

// touch grass
use grass::{Options, OutputStyle};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::path::Path;
use std::result::Result;
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
pub struct SiteCfg {
    pub themes: HashMap<String, Theme>
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
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir(path).expect("Could not create directory static/favicons/");
    }
    Ok(())
}

fn main() {
    let sitecfg: SiteCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();

    // Step 1.1: Generate _themes.scss
    let mut maprows: String = String::from("");
    let themeiter: Vec<(String, Theme)> = sitecfg.themes.into_iter().collect();

    for (name, value) in &themeiter {
        let mut colorbytes = [0u8; 3];
        let mut fgcolorbytes = [0u8; 3];
        hex::decode_to_slice(&value.color, &mut colorbytes as &mut [u8]).unwrap();
        hex::decode_to_slice(&value.fgcolor, &mut fgcolorbytes as &mut [u8]).unwrap();
        maprows.push_str(&format!("    '{}': ('theme': #{}, 'fgcolor': #{}, 'themergb': rgb({}, {}, {}), 'fgcolorrgb': rgb({}, {}, {})),\n", name, value.color, value.fgcolor, colorbytes[0], colorbytes[1], colorbytes[2], fgcolorbytes[0], fgcolorbytes[1], fgcolorbytes[2]));
    }

    /*
    -- Example output --
    $themes: (
        "gold": ("theme": "#F0D000", "fgcolor": "#000000", "themergb": "rgb(240, 208, 0)", "fgcolorrgb": "rgb(0,0,0)")
        ...
    );
     */

    let thememap = &format!("$themes: (\n{}\n);", maprows);
    std::fs::write("src/styling/_themes.scss", thememap).unwrap();

    // Step 1.2: Build base CSS
    let grass_options: Options = Options::default().style(OutputStyle::Compressed);
    let basescss = read_file("./src/styling/base.scss").unwrap();
    let basecss = &grass::from_string(basescss, &grass_options).unwrap();

    std::fs::write("./static/base.css", basecss).unwrap();

    // Step 1.3: Build theme CSS
    let themescss = read_file("./src/styling/theme.scss").unwrap();
    mkdir("static/themes/").unwrap();
    for (name, _value) in &themeiter {
        // Separate variable to store the modified SCSS so theme.scss is not loaded every time
        let themescssvar = &themescss.clone().replace("/* $themename */", &format!("$themename: '{}';", name));
        let themecss = &grass::from_string(themescssvar, &grass_options).unwrap();
        std::fs::write(format!("static/themes/{}.css", name), themecss).unwrap();
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

    for (key, value) in &themeiter {
        // It is unlikely that default favicons would change so to reduce build time and disk writes, generation is skipped if the favicon already exists.
        if !file_exists("static/favicons/default_{key}.ico") {
            let mut image = RgbImage::new(16, 16);
            for x in 0..16 {
                for y in 0..16 {
                    let mut bytes = [0u8; 3];
                    hex::decode_to_slice(&value.color, &mut bytes as &mut [u8]).unwrap();
                    image.put_pixel(x, y, Rgb(bytes));
                }
            }
            image.save(format!("static/favicons/default_{key}.ico")).unwrap_or_else(|_| panic!("Error while saving file default_{key}.ico"))
        }
    }

}