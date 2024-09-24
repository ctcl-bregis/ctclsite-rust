// ctclsite-rust - CTCL 2020-2024
// File: src/lib.rs
// Purpose: Theme-related types and functions
// Created: September 21, 2024
// Modified: September 21, 2024

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct FontVariant {
    pub style: String,
    pub weight: String,
    pub formats: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FontFamily {
    // Internal name
    pub name: String,
    // Display name
    pub dispname: String,
    // Font definitions
    pub styles: HashMap<String, FontVariant>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Theme {
    // Theme color
    pub color: String,
    // Color of text that on a background of the theme color
    pub fgcolor: String,
    // Theme color in RGB
    #[serde(skip_deserializing, default = "emptytripleu8")]
    pub colorrgb: [u8; 3],
    // Color of text that on a background of the theme color
    #[serde(skip_deserializing, default = "emptytripleu8")]
    pub fgcolorrgb: [u8; 3],
    // Default font to be used in themes
    pub defaultfont: String,
    // Styling to use
    #[serde(default = "emptystring")]
    pub css: String,
    // To be filled by loadthemes
    #[serde(skip_deserializing, default = "emptystring")]
    pub rendered: String
}

pub fn loadfonts(sitecfg: &SiteConfig) -> Result<HashMap<String, FontFamily>, Error> {
    let mut fonts: HashMap<String, FontFamily> = HashMap::new();
    let fontroot = sitecfg.fontdir.clone();

    mkdir("static/fonts/")?;

    for entry in WalkDir::new(&fontroot).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            let name = entry
                .path()
                .display()
                .to_string()
                .strip_prefix(&fontroot)
                .unwrap()
                .to_string();
            let configpathstr = format!("{}/font.json", entry.path().display()).clone();
            let configpath = Path::new(&configpathstr);

            // Prevent "" from being added
            if !name.is_empty() {  
                let font: FontFamily = match configpath.exists() {
                    true => serde_json::from_str(&read_file(configpath).unwrap()).unwrap(),
                    false => continue
                };

                match mkdir(&format!("static/fonts/{}", font.name)) {
                    Ok(_) => (),
                    Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error creating directory {}: {e}", &format!("static/fonts/{}", font.name))))
                }

                for (style, styledata) in &font.styles {
                    match mkdir(&format!("static/fonts/{}/{}", font.name, style)) {
                        Ok(_) => (),
                        Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error creating directory {}: {e}", &format!("static/fonts/{}", font.name))))
                    }
                    for filename in styledata.formats.values() {
                        // Define paths here since using format! in arguments was getting out of hand
                        let source = &format!("{}/{}", entry.path().display(), filename);
                        let target = &format!("static/fonts/{}/{}/{}", font.name, style, filename);

                        match std::fs::copy(source, target) {
                            Ok(_) => (),
                            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error copying {} to {}: {e}", source, target))),
                        }
                    }
                }

                fonts.insert(font.name.clone(), font);
            }
        }
    }


    Ok(fonts)
}

pub fn loadthemes(sitecfg: &SiteConfig) -> Result<HashMap<String, Theme>, Error> {
    let themeroot = &sitecfg.themedir.clone();

    // At this point, static/ should exist
    mkdir("static/styles/")?;

    let mut themes: HashMap<String, Theme> = HashMap::new();
    for entry in WalkDir::new(themeroot).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            let name = entry
                .path()
                .display()
                .to_string()
                .strip_prefix(themeroot)
                .unwrap()
                .to_string();
            // Prevent "" and "_defaults" from being added
            if !name.is_empty() && name != "_defaults" {            
                let mut theme: Theme = serde_json::from_str(&read_file(format!("{}/theme.json", entry.path().display())).unwrap()).unwrap();
                let mut colorrgb = [0u8; 3];
                hex::decode_to_slice(theme.color.replace('#', ""), &mut colorrgb as &mut [u8]).unwrap();
                let mut fgcolorrgb = [0u8; 3];
                hex::decode_to_slice(theme.fgcolor.replace('#', ""), &mut fgcolorrgb as &mut [u8]).unwrap();
        
                theme.colorrgb = colorrgb;
                theme.fgcolorrgb = fgcolorrgb;

                themes.insert(name, theme);
            }
        }
    }

    let mut ctx = Context::new();
    ctx.insert("themes", &themes);
    ctx.insert("fonts", &sitecfg.fonts);

    if sitecfg.themevars.is_some() {
        for (key, value) in sitecfg.themevars.as_ref().unwrap() {
            ctx.insert(key, &value);
        }        
    }

    let mut renderedthemes: HashMap<String, Theme> = HashMap::new();
    for (themename, theme) in themes.iter() {

        // For some reason, Tera does not register anything with the .lisc extension (or any unrecognized extension?) when using a glob so files must be added "manually"
        let mut tmpl = Tera::default();

        let searchpath = if theme.css.is_empty() || theme.css == "defaults" {
            format!("{themeroot}_defaults/")
        } else {
            format!("{themeroot}{themename}/")
        };

        for entry in WalkDir::new(searchpath).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() && entry.path().extension().unwrap() == "lisc" {
                // Have the name of the template just the file name
                let _ = tmpl.add_template_file(entry.path(), entry.path().file_name().unwrap().to_str());
            }
        }

        ctx.insert("theme", theme);
        
        let rendered = match tmpl.render("dark.lisc", &ctx) {
            Ok(t) => t,
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        };
        let csspath = format!("static/styles/{themename}.css");

        write_file(&csspath, &rendered)?;

        let renderedtheme: Theme = Theme {
            color: theme.color.clone(),
            fgcolor: theme.fgcolor.clone(),
            colorrgb: theme.colorrgb,
            fgcolorrgb: theme.fgcolorrgb,
            css: theme.css.clone(),
            defaultfont: theme.defaultfont.clone(),
            rendered: format!("/{csspath}"),
        };

        renderedthemes.insert(themename.clone(), renderedtheme);
        
    }

    Ok(renderedthemes)
}