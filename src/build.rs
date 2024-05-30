// ctclsite-rust - CTCL 2020-2024
// File: src/build.rs
// Purpose: Build needed files
// Created: February 28, 2024
// Modified: May 29, 2024

// touch grass
use grass::{Options, OutputStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::write;
use std::fs::read;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::path::Path;
use std::result::Result;
use minifier::js::minify;
extern crate image;
use image::{Rgb, RgbImage};
use chrono::Utc;

#[derive(Deserialize, Serialize)]
struct Theme {
    color: String,
    altcolor: Option<String>,
    fgcolor: String
}

#[derive(Deserialize, Serialize)]
struct Sitecfg {
    scss: String,
    themes: HashMap<String, Theme>,
    pages: HashMap<String, String>
}

pub fn get_system() -> String {
    let system: String = if std::env::var("hwcodename").is_ok() {
        format!("{} \"{}\"", gethostname::gethostname().to_str().unwrap(), std::env::var("hwcodename").unwrap())
    } else {
        gethostname::gethostname().to_str().unwrap().to_string()
    };
    system
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

fn main() {
    let sitecfg: Sitecfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();

    // Step 1: Generate _themes.scss
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

    // Step 2: Build base CSS
    let grass_options: Options = Options::default().style(OutputStyle::Compressed);
    let basescss = read_file("./src/styling/base.scss").unwrap();
    let basecss = &grass::from_string(basescss, &grass_options).unwrap();

    std::fs::write("./static/base.css", basecss).unwrap();

    // Step 3: Build theme CSS
    let themescss = read_file("./src/styling/theme.scss").unwrap();
    for (name, _value) in &themeiter {
        // Separate variable to store the modified SCSS so theme.scss is not loaded every time
        let themescssvar = &themescss.clone().replace("/* $themename */", &format!("$themename: '{}';", name));
        let themecss = &grass::from_string(themescssvar, &grass_options).unwrap();
        std::fs::write(format!("static/{}.css", name), themecss).unwrap();
    }

    // Step 3: Generate default favicons
    if !std::path::Path::new("static/favicons/").exists() {
        std::fs::create_dir("static/favicons/").expect("Could not create directory static/favicons/");
    }
 
    for (key, value) in &themeiter {
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