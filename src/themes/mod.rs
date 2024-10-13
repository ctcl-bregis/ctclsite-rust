// ctclsite - CTCL 2019-2024
// File: src/themes/mod.rs
// Purpose: Theme module
// Created: September 21, 2024
// Modified: October 13, 2024

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
    #[serde(default = "defaulttrue")]
    pub enabled: bool,
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
    // Use CSS templates from _defaults or custom templates from the theme's directory
    pub usedefaults: bool,
    // If specified (Some), use the favicon from the URL specified; If not specified (None), generate a favicon using the theme color
    pub favicon: Option<String>,
    // To be filled by loadthemes
    #[serde(skip_deserializing, default = "emptystring")]
    pub rendered: String
}

pub fn loadfonts(sitecfg: &SiteConfig) -> Result<HashMap<String, FontFamily>, Error> {
    let mut fonts: HashMap<String, FontFamily> = HashMap::new();
    let fontroot = sitecfg.fontdir.clone();

    mkdir("static/fonts/")?;

    for entry in WalkDir::new(&fontroot).into_iter().filter_map(|e| e.ok()) {
        if !entry.path().is_dir() {
            continue
        }

        let name = entry
            .path()
            .display()
            .to_string()
            .strip_prefix(&fontroot)
            .unwrap()
            .to_string();

        let configpath = entry.path().join("font.json");

        // Prevent "" from being added
        if name.is_empty() {
            continue
        }

        let font: FontFamily = match fs::exists(&configpath) {
            Ok(b) => match b {
                true => serde_json::from_str(&read_file(configpath).unwrap()).unwrap(),
                false => continue
            },
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
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

    Ok(fonts)
}

pub fn loadthemes(sitecfg: &SiteConfig) -> Result<HashMap<String, Theme>, Error> {
    // One must imagine borrow checker happy
    let themeroot = &sitecfg.themedir.clone();

    mkdir("static/styles/")?;

    let mut themeconfs: HashMap<String, Theme> = HashMap::new();

    for entry in WalkDir::new(themeroot).into_iter().filter_map(|e| e.ok()) {
        if !entry.path().is_dir() {
            continue    
        }

        if entry.path().to_string_lossy() == *themeroot {
            continue
        }

        let name = entry.path().file_name().unwrap().to_string_lossy();
        
        // Prevent "" and "_defaults" from being added
        if name.is_empty() || name == "_defaults" {            
            continue
        }

        let themejson = &read_file(entry.path().join("theme.json"))?;
        let mut themeconf: Theme = serde_json::from_str(themejson)?;

        if !themeconf.enabled {
            debug!("loadthemes: theme \"{}\" is disabled from its configuration file", name);
            continue
        }

        let mut colorrgb = [0u8; 3];
        hex::decode_to_slice(themeconf.color.replace('#', ""), &mut colorrgb as &mut [u8]).unwrap();
        let mut fgcolorrgb = [0u8; 3];
        hex::decode_to_slice(themeconf.fgcolor.replace('#', ""), &mut fgcolorrgb as &mut [u8]).unwrap();

        themeconf.colorrgb = colorrgb;
        themeconf.fgcolorrgb = fgcolorrgb;

        themeconfs.insert(name.to_string(), themeconf);
    }

    // Much of the data is shared across themes so ctx is defined once then modified
    let mut ctx = Context::new();
    // Variables that are constant across themes
    // Themes need data about each other
    ctx.insert("themes", &themeconfs);
    ctx.insert("fonts", &sitecfg.fonts);

    if sitecfg.themevars.is_some() {
        for (key, value) in sitecfg.themevars.clone().unwrap().iter() {
            ctx.insert(key, value);
        }
    }

    let mut newthemes: HashMap<String, Theme> = HashMap::new();
    
    for (name, theme) in themeconfs.iter() {
        let tmpl = match theme.usedefaults {
            true => Lysine::new(&format!("{}/_defaults/**/*.lisc", &themeroot)).unwrap(),
            false => Lysine::new(&format!("{}/{}/**/*.lisc", &themeroot, &name)).unwrap()
        };

        ctx.insert("theme", theme);
        
        let rendered = match tmpl.render("dark.lisc", &ctx) {
            Ok(t) => t,
            // TODO: somehow get a descriptive error from Lysine
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("when rendering theme {}: {:?}", name, e)))
        };
        let csspath = format!("static/styles/{name}.css");

        write_file(&csspath, &rendered)?; 

        let newtheme: Theme = Theme {
            enabled: true,
            color: theme.color.clone(),
            fgcolor: theme.fgcolor.clone(),
            colorrgb: theme.colorrgb,
            fgcolorrgb: theme.fgcolorrgb,
            defaultfont: theme.defaultfont.clone(),
            usedefaults: theme.usedefaults,
            favicon: theme.favicon.clone(),
            rendered: format!("/{csspath}"),
        };

        newthemes.insert(name.clone(), newtheme);
    }

    Ok(newthemes)

}