// ctclsite - CTCL 2019-2024
// File: src/themes/mod.rs
// Purpose: Theme module
// Created: September 21, 2024
// Modified: October 15, 2024

use serde::{Serialize, Deserialize};
use minifier::css;

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
    // Fallback font to use
    pub fallback: String,
    // Font definitions
    pub styles: HashMap<String, FontVariant>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
    // Use CSS templates from _defaults (true) or custom templates from the theme's directory (false)
    pub defaults: bool,
    // To be filled by loadthemes
    #[serde(skip_deserializing, default = "emptystring")]
    pub favicon: String,
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

        // Panic if the path does not have fontroot as a prefix as it always should have it
        let name = entry.path().strip_prefix(&fontroot).unwrap();

        let configpath = entry.path().join("font.json");

        // Prevent "" from being added
        if name.to_string_lossy().is_empty() {
            continue
        }

        match fs::exists(&configpath) {
            Ok(b) => match b {
                true => (),
                false => continue
            },
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        };

        let fontjson = match read_file(&configpath) {
            Ok(c) => c,
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        };

        let font: FontFamily = match serde_json::from_str(&fontjson) {
            Ok(c) => c,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error loading JSON file for font {}: {e}", name.to_string_lossy())))
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

pub fn mkfavicons(themes: &HashMap<String, Theme>) -> Result<(), Error> {
    // At this point, static/ should exist
    mkdir("static/favicons/")?;

    for (name, theme) in themes.iter() {
        // It is unlikely that default favicons would change so to reduce build time and disk writes, generation is skipped if the favicon already exists.
        if !fs::exists("static/favicons/default_{name}.ico").unwrap() {
            let mut image = RgbImage::new(16, 16);
            let mut bytes = [0u8; 3];
            hex::decode_to_slice(theme.color.replace('#', ""), &mut bytes as &mut [u8]).unwrap();
            for x in 0..16 {
                for y in 0..16 {
                    image.put_pixel(x, y, Rgb(bytes));
                }
            }
            image.save(format!("static/favicons/default_{name}.ico")).unwrap_or_else(|_| panic!("Error while saving file default_{name}.ico"))
        }
    }

    Ok(())
}

pub fn loadthemes(sitecfg: &SiteConfig) -> Result<HashMap<String, Theme>, Error> {
    // One must imagine borrow checker happy
    let themeroot = &sitecfg.themedir.clone();

    mkdir("static/styles/")?;

    let mut newthemes: HashMap<String, Theme> = HashMap::new();

    for (name, theme) in sitecfg.themes.iter() {
        let mut newtheme = theme.clone();

        // This is needed until a hex2rgb function is added to Lysine
        hex::decode_to_slice(theme.color.replace('#', ""), &mut newtheme.colorrgb as &mut [u8]).unwrap();
        hex::decode_to_slice(theme.fgcolor.replace('#', ""), &mut newtheme.fgcolorrgb as &mut [u8]).unwrap();
        
        let defaultfontfamily = sitecfg.fonts.get(&theme.defaultfont).unwrap();
        let defaultfont = format!("{}; {}", defaultfontfamily.dispname, defaultfontfamily.fallback);

        let newtheme: Theme = Theme {
            enabled: true,
            color: theme.color.clone(),
            fgcolor: theme.fgcolor.clone(),
            colorrgb: theme.colorrgb,
            fgcolorrgb: theme.fgcolorrgb,
            defaultfont,
            defaults: theme.defaults,
            favicon: theme.favicon.clone(),
            rendered: String::from(""),
        };

        newthemes.insert(name.clone(), newtheme);
    }

    mkfavicons(&newthemes)?;

    let mut ctx = Context::new();
    ctx.insert("themes", &newthemes);
    ctx.insert("fonts", &sitecfg.fonts);

    if sitecfg.themevars.is_some() {
        for (key, value) in sitecfg.themevars.clone().unwrap().iter() {
            ctx.insert(key, value);
        }
    }

    // Due to the pagebutton theming, themes need to know the data of other themes. This requires the theme map to be iterated through again.
    // TODO: Is there a way to do this without cloning newthemes?
    for (name, theme) in newthemes.clone() {
        let tmpl = match theme.defaults {
            true => Lysine::new(&format!("{}/_defaults/**/*.lisc", &themeroot)).unwrap(),
            false => Lysine::new(&format!("{}/{}/**/*.lisc", &themeroot, &name)).unwrap()
        };

        ctx.insert("theme", &theme);

        // TODO: Have the theme select the template to use
        let mut rendered = match tmpl.render("dark.lisc", &ctx) {
            Ok(t) => t,
            // TODO: somehow get a descriptive error from Lysine
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("when rendering theme {}: {:?}", name, e)))
        };

        if sitecfg.minimizecss {
            rendered = minifier::css::minify(&rendered).unwrap().to_string();
            write_file(format!("static/styles/{}.css", &name), &rendered)?;
        } else {
            write_file(format!("static/styles/{}.css", &name), &rendered)?;
        }

        newthemes.get_mut(&name).unwrap().rendered = format!("/static/styles/{}.css", &name);
    }

    Ok(newthemes)
}
