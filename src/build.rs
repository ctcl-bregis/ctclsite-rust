// ctclsite-rust - CTCL 2022-2024
// File: src/build.rs
// Purpose: Build needed files
// Created: February 28, 2024
// Modified: March 29, 2024

// touch grass
use grass::{Options, OutputStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::result::Result;
use minifier::js::minify;
extern crate image;
use image::{Rgb, RgbImage};
use chrono::Utc;

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
    let grass_options: Options = Options::default()
    .style(OutputStyle::Compressed);

    // Step 1: Build the CSS for each theme
    let sitecfg: Sitecfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();
    let sitecfgthemes: HashMap<String, Theme> = sitecfg.themes;
 
    let mbase_scss_content = read_file("src/styling/common.scss").unwrap();
    let mbase_css = grass::from_string(mbase_scss_content, &grass_options).unwrap();
 
    let mtheme_scss_content = read_file("src/styling/theme.scss").unwrap();
 
    for key in sitecfgthemes.keys() {
        // Do not overwrite the original string
        let mut themescss = mtheme_scss_content.clone();
 
        // Grass apparently does not have the "Inject Values" function that pyScss has so a hack is required here
        let mut scssvalues = String::from("$themes: (\n");
        for (key2, value2) in &sitecfgthemes {
            scssvalues.push_str(&format!("'{}': ('color': {}, 'fgcolor': {}),\n", key2, value2.color, value2.fgcolor));
            let mut colorbytes = [0u8; 3];
            let mut fgcolorbytes = [0u8; 3];
            hex::decode_to_slice(remove_first(&value2.color).unwrap(), &mut colorbytes as &mut [u8]).unwrap();
            hex::decode_to_slice(remove_first(&value2.color).unwrap(), &mut fgcolorbytes as &mut [u8]).unwrap();
            scssvalues.push_str(&format!("'{}rgb': ('color': rgb({}, {}, {}), 'fgcolor': rgb({}, {}, {})),\n", key2, colorbytes[0], colorbytes[1], colorbytes[2], fgcolorbytes[0], fgcolorbytes[1], fgcolorbytes[2]));
        }
        scssvalues.push_str(");");
 
        scssvalues.push_str(&format!("$theme: {};\n$fgcolor: {}; \n", format_args!("map.get(map.get($themes, '{}'), 'color')", key), format_args!("map.get(map.get($themes, '{}'), 'fgcolor')", key)));
        scssvalues.push_str(&format!("$themergb: {};\n$fgcolorgb: {}; \n", format_args!("map.get(map.get($themes, '{}rgb'), 'color')", key), format_args!("map.get(map.get($themes, '{}rgb'), 'fgcolor')", key)));
 
        themescss = themescss.replace("$$thememap$$", &scssvalues);
 
        let mut themecss = mbase_css.clone();
 
        themecss.push_str(&grass::from_string(themescss, &grass_options).unwrap());
 
        let current_time = Utc::now().format("%b %-d, %Y, %H:%M %Z");
        let system: String = if std::env::var("hwcodename").is_ok() {
            format!("{} \"{}\"", gethostname::gethostname().to_str().unwrap(), std::env::var("hwcodename").unwrap())
        } else {
            gethostname::gethostname().to_str().unwrap().to_string()
        };

        let mut header = format!("/*\nctclsite-rust styling for theme \"{}\"\nGenerated {} on system {} \n*/\n", &key, current_time, system);
        header.push_str(&themecss);

        std::fs::write(format!("./static/{}.css", &key), &header).expect("Unable to write file themes.json");
    }
 
    // Step 2: Minimize and move JavaScript
    let paths = std::fs::read_dir("src/js/").unwrap();

    for path in paths {
        let upath = path.unwrap().file_name();
        let filename = upath.to_str().unwrap();
        let jsfile: String;
        if filename.ends_with(".js") {
            jsfile = read_file(&format!("src/js/{}", filename)).expect("Error reading src/js/clientinfo.js");
            let minjsfile = minify(&jsfile).to_string();
            std::fs::write(&format!("static/{}", filename), minjsfile).unwrap();
        }
    }
 
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
        image.save(format!("static/favicons/default_{key}.ico")).unwrap_or_else(|_| panic!("Error while saving file default_{key}.ico"))
    }
}