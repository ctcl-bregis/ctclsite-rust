// ctclsite-rust - CTCL 2022-2024
// File: src/build.rs
// Purpose: Build needed files
// Created: February 28, 2024
// Modified: March 6, 2024

// touch grass
use grass;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::result::Result;
use minifier::js::minify;
extern crate image;
use image::{Rgb, RgbImage};

#[derive(Deserialize, Serialize)]
struct Theme {
    color: String,
    fgcolor: String
}

#[derive(Deserialize, Serialize)]
struct Navitem {
    navtype: String,
    app: String,
    title: String,
    link: String,
    float: String,
}

#[derive(Deserialize, Serialize)]
struct Sitecfg {
    scss: String,
    themes: HashMap<String, Theme>,
    pages: HashMap<String, String>
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();

    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

fn main() {

    let grass_options: grass::Options = grass::Options::default()
        .style(grass::OutputStyle::Compressed);

    // Step 1: Build the CSS for each theme
    let sitecfg: Sitecfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();
    let sitecfgthemes: HashMap<String, Theme> = sitecfg.themes;

    let mut themecfg: HashMap<String, String> = HashMap::new();

    let mbase_scss_content = read_file("src/styling/common.scss").unwrap();
    let mbase_css = grass::from_string(mbase_scss_content, &grass_options).unwrap();

    let mtheme_scss_content = read_file("src/styling/theme.scss").unwrap();

    for (key, value) in &sitecfgthemes {
        // Do not overwrite the original string
        let mut themescss = mtheme_scss_content.clone();

        // Grass apparently does not have the "Inject Values" function that pyScss has so a hack is required here
        let mut scssvalues = format!("$theme: {};\n$fgcolor: {};\n", value.color, value.fgcolor);
        
        scssvalues.push_str("$themes: (\n");
        for (key2, value2) in &sitecfgthemes {
            scssvalues.push_str(&format!("'{}': ('color': '{}', 'fgcolor': '{}'),\n", key2, value2.color, value2.fgcolor));
        }
        scssvalues.push_str(");");

        themescss = themescss.replace("$$thememap$$", &scssvalues);

        let mut themecss = mbase_css.clone();

        themecss.push_str(&grass::from_string(themescss, &grass_options).unwrap());

        themecfg.insert(key.to_owned(), themecss);
    }

    let json = serde_json::to_string(&themecfg).unwrap();

    std::fs::write("./themes.json", json).expect("Unable to write file themes.json");

    // Step 2: Minimize and move JavaScript
    let clientinfojs = &read_file("src/js/clientinfo.js").expect("Error reading src/js/clientinfo.js");
    let commonjs = &read_file("src/js/common.js").expect("Error reading src/js/common.js");
    let minclientinfojs = minify(clientinfojs).to_string();
    let mincommmonjs = minify(commonjs).to_string();

    std::fs::write("static/clientinfo.js", minclientinfojs).unwrap();
    std::fs::write("static/common.js", mincommmonjs).unwrap();

    // Step 3: Generate default favicons
    if !std::path::Path::new("static/favicons/").exists() {
        std::fs::create_dir("static/favicons/").expect("Could not create directory static/favicons/");
    }

    for (key, value) in &sitecfgthemes {
        let mut image = RgbImage::new(16, 16);
        for x in 0..16 {
            for y in 0..16 {
                let mut bytes = [0u8; 3];
                hex::decode_to_slice(remove_first(&value.color).unwrap(), &mut bytes as &mut [u8]).unwrap();
                image.put_pixel(x, y, Rgb(bytes));
            }
        }
        image.save(format!("static/favicons/default_{key}.ico")).expect(&format!("Error while saving file default_{key}.ico"));
    }
}