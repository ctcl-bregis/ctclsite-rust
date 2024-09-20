// ctclsite-rust - CTCL 2020-2024
// File: src/lib.rs
// Purpose: Module import and commonly used functions
// Created: November 28, 2022
// Modified: September 20, 2024

//use minifier::js;
//use minify_html::{minify, Cfg};
use comrak::{markdown_to_html, Options};
use image::{Rgb, RgbImage};
use indexmap::IndexMap;
use log::{debug, error, info, warn};
use serde_json::value::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Error, ErrorKind};
use std::path::Path;
use std::result::Result;
use tera::{Context, Tera};
use walkdir::WalkDir;

// To-Do: This file is long, consider splitting some code into modules

// ----------------------------
// serde defaults

pub fn emptystringindexmap() -> IndexMap<String, String> {
    let map: IndexMap<String, String> = IndexMap::new();
    map
}

pub fn emptypagehashmap() -> HashMap<String, Page> {
    let map: HashMap<String, Page> = HashMap::new();
    map
}

pub fn emptythemehashmap() -> HashMap<String, Theme> {
    let map: HashMap<String, Theme> = HashMap::new();
    map
}

pub fn emptyfonthashmap() -> HashMap<String, FontFamily> {
    let map: HashMap<String, FontFamily> = HashMap::new();
    map
}

pub fn emptynavbarlinkhashmap() -> HashMap<String, NavbarLink> {
    let map: HashMap<String, NavbarLink> = HashMap::new();
    map
}

pub fn emptystring() -> String {
    String::new()
}

pub fn emptystringvec() -> Vec<String> {
    let vec: Vec<String> = Vec::new();
    vec
}

pub fn emptyusizevec() -> Vec<usize> {
    let vec: Vec<usize> = Vec::new();
    vec
}

pub fn emptytripleu8() -> [u8; 3] {
    let vec: [u8; 3] = [0u8; 3];
    vec
}

pub fn defaultfalse() -> bool {
    false
}

pub fn defaulttrue() -> bool {
    true
}

// ----------------------------

// Data from clientinfo.js
#[derive(Deserialize, Serialize)]
pub struct ClientLogEntry {
    pub uuid: String,
    pub url_path: String,
    pub timezone: String,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub cpu_cores: String,
    pub mem_size: String,
    pub max_tp: String,
    pub oscpu: String,
    pub plat: String,
    pub screen_x: String,
    pub screen_y: String,
    pub screen_pix_ratio: String,
    pub screen_pix_depth: String,
    pub canvas_fp: String,
    pub online: String,
    pub pdf_viewer: String,
    pub cookies_enabled: String,
    pub dnt: String,
    pub langs: String,
    pub prod: String,
    pub prod_sub: String,
    pub client_user_agent: String,
    pub vend: String,
    pub inner_height: String,
    pub inner_width: String
}

// ClientLogEntry data + data collected server-side
#[derive(Deserialize, Serialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub ip: String,
    pub useragent: String,
    pub clientdata: Option<ClientLogEntry>
}

#[derive(Deserialize, Serialize)]
pub struct NavbarLink {
    title: String,
    link: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FontVariant {
    style: String,
    weight: String,
    formats: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FontFamily {
    // Internal name
    name: String,
    // Display name
    dispname: String,
    // Font definitions
    styles: HashMap<String, FontVariant>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Theme {
    // Theme color
    color: String,
    // Color of text that on a background of the theme color
    fgcolor: String,
    // Theme color in RGB
    #[serde(skip_deserializing, default = "emptytripleu8")]
    colorrgb: [u8; 3],
    // Color of text that on a background of the theme color
    #[serde(skip_deserializing, default = "emptytripleu8")]
    fgcolorrgb: [u8; 3],
    // Default font to be used in themes
    defaultfont: String,
    // Styling to use
    #[serde(default = "emptystring")]
    css: String,
    // To be filled by loadthemes
    #[serde(skip_deserializing, default = "emptystring")]
    rendered: String
}

#[derive(Deserialize, Serialize)]
pub enum ExtensionFileType {
    #[serde(alias = "binary")]
    Binary,
    // Anything with the "config" type shall not be copied to "static/"
    #[serde(alias = "config")]
    Config,
    #[serde(alias = "image")]
    Image,
    #[serde(alias = "pdf")]
    Pdf,
    #[serde(alias = "text")]
    Text,
    #[serde(alias = "video")]
    Video
}

#[derive(Deserialize, Serialize)]
pub struct SiteConfig {
    // IP to bind to, usually either "0.0.0.0" or "127.0.0.1"
    pub bindip: String,
    // Port to bind to
    pub bindport: u16,
    // Website domain, for example: "ctcl.lgbt". Currently used for the "link" meta tag.
    pub sitedomain: String,
    // Directory to scan for font directories
    pub fontdir: String,
    // Directory to scan for JavaScript files
    pub jsdir: String,
    // Directory to scan for page directories
    pub pagedir: String,
    // Directory to scan for globally-used static files
    pub staticdir: String,
    // Directory to scan for theme directories
    pub themedir: String,
    // Directory to scan for HTML templates
    pub templatedir: String,
    // Theme to default to if a specific theme is not defined. It must refer to an existing theme.
    pub defaulttheme: String,
    // How many "workers" to be deployed by Actix Web
    pub cpus: usize,
    // Map of links to redirect to
    pub redirects: HashMap<String, String>,
    // Disable memcache client feature altogether
    pub enablememcache: bool,
    // URL to memcache server
    pub memcache: String,
    // Links to make available in the navbar
    pub navbar: Vec<NavbarLink>,
    // Exists solely for debugging purposes
    pub minimizehtml: bool,
    // Definition of file types by file extension, used by collectstatic to determine what files to copy and may be used for the upcoming file viewer feature
    pub filetypes: HashMap<String, ExtensionFileType>,
    // Optional: Any extra parameters defined in config.json to be available in Lysine/Tera CSS templates
    pub themevars: Option<HashMap<String, Value>>,
    // Optional: Any extra parameters defined in config.json to be available in HTML templates
    pub uservars: Option<HashMap<String, Value>>,
    // Skip since pages, themes and fonts should not be defined in config.json
    #[serde(skip_deserializing, default = "emptypagehashmap")]
    pub pages: HashMap<String, Page>,
    #[serde(skip_deserializing, default = "emptythemehashmap")]
    pub themes: HashMap<String, Theme>,
    #[serde(skip_deserializing, default = "emptyfonthashmap")]
    pub fonts: HashMap<String, FontFamily>
}

// Partial config that only has fields for things required to start the webserver to avoid loading all of the pages twice
#[derive(Deserialize, Serialize)]
pub struct PartialSiteConfig {
    pub bindip: String,
    pub bindport: u16,
    pub cpus: usize
}

// -- Page Type Definitions --

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkFullCustom {
    link: String,
    theme: String,
    startdate: String,
    enddate: String,
    title: String,
    desc: String,
    icon: String,
    icontitle: String,
    cat: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkFull {
    page: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkTitleOnlyCustom {
    title: String,
    theme: String,
    link: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkTitleOnly {
    page: String
}



// To-Do: Have the ability to specifiy if a link uses data (theme, description, etc.) from a page configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum LinkType {
    #[serde(rename(serialize = "full", deserialize = "full"))]
    Full(LinkFull),
    #[serde(rename(serialize = "fullcustom", deserialize = "fullcustom"))]
    FullCustom(LinkFullCustom),
    #[serde(rename(serialize = "titleonly", deserialize = "titleonly"))]
    TitleOnly(LinkTitleOnly),
    #[serde(rename(serialize = "titleonlycustom", deserialize = "titleonlycustom"))]
    TitleOnlyCustom(LinkTitleOnlyCustom)
}

// Weird name, I know
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentContent {
    boxed: bool,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    title: String,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    theme: String,
    #[serde(default = "defaultfalse")]
    fitscreen: bool,
    content: String,
    #[serde(skip_deserializing, default = "emptystring")]
    rendered: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentLinklist {
    boxed: bool,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    title: String,
    // Unused if boxed is false
    #[serde(default = "emptystring")]
    theme: String,
    #[serde(default = "defaultfalse")]
    fitscreen: bool,
    links: Vec<LinkType>,
    #[serde(skip_deserializing, default = "emptystring")]
    rendered: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(alias = "linklist")]
    Linklist(ContentLinklist),
    #[serde(alias = "content")]
    Content(ContentContent)
}


// All pages are "Sections" now
// To replicate a "Content" page from ctclsite-rust v1, there would be one section with "boxed" 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    // -- Commonly defined fields --
    // Title of the page that is displayed in the <title></title> tag, the page header and meta tags
    pub title: String,
    // If not defined, this value is to be replaced with the default theme specified in <config root>/config.json
    #[serde(default = "emptystring")]
    pub theme: String,
    // Currently, this can be anything. For example: "2024", "????", "February 22, 2022"
    #[serde(default = "emptystring")]
    pub startdate: String,
    // Currently, this can be anything. For example: "2024", "????", "February 22, 2022"
    // Unused if startdate is undefined or empty
    #[serde(default = "emptystring")]
    pub enddate: String,
    // Description to show in meta tags and Linklist entries
    #[serde(default = "emptystring")]
    pub desc: String,
    // Icon to show on Linklist entries; not the page favicon
    #[serde(default = "emptystring")]
    pub icon: String,
    // Mouseover text for icons. Unused if icon is not defined or empty.
    #[serde(default = "emptystring")]
    pub icontitle: String,
    // Keywords for pages
    #[serde(default = "emptystringvec")]
    pub keywords: Vec<String>,
    // Category that a page has in a Linklist entry
    #[serde(default = "emptystring")]
    pub cat: String,
    // If not defined, the HTML template should use the default favicon that corresponds with the page theme
    #[serde(default = "emptystring")]
    pub favicon: String,
    // Should be set to "false" with pages like About and Services that are made up of boxed sections
    #[serde(default = "defaulttrue")]
    pub headerids: bool,
    #[serde(default = "defaulttrue")]
    pub shownavbar: bool,
    // Definitions of content to be processed. This value should not be used directly in templates.
    pub content: IndexMap<String, Content>,
    // Any extra parameters defined in configuration files to be available in HTML templates
    pub uservars: Option<HashMap<String, Value>>
}

pub fn mkfavicons(themes: &HashMap<String, Theme>) -> Result<(), Error> {
    // At this point, static/ should exist
    mkdir("static/favicons/")?;

    for (key, value) in themes {
        // It is unlikely that default favicons would change so to reduce build time and disk writes, generation is skipped if the favicon already exists.
        if !fs::exists("static/favicons/default_{key}.ico").unwrap() {
            let mut image = RgbImage::new(16, 16);
            for x in 0..16 {
                for y in 0..16 {
                    let mut bytes = [0u8; 3];
                    hex::decode_to_slice(value.color.replace('#', ""), &mut bytes as &mut [u8]).unwrap();
                    image.put_pixel(x, y, Rgb(bytes));
                }
            }
            image.save(format!("static/favicons/default_{key}.ico")).unwrap_or_else(|_| panic!("Error while saving file default_{key}.ico"))
        }
    }

    Ok(())
}

pub fn buildjs(sitecfg: &SiteConfig) -> Result<(), Error> {
    mkdir("static/js/")?;

    match fs::read_dir(&sitecfg.jsdir) {
        Ok(d) => {
            for entry in d {
                match entry {
                    Ok(rd) => match fs::copy(rd.path(), format!("static/{}", rd.path().to_string_lossy())) {
                        Ok(_) => (),
                        Err(ce) => return Err(Error::new(ErrorKind::Other, format!("{ce}")))
                    }
                    Err(re) => return Err(Error::new(ErrorKind::Other, format!("{re}")))
                }
            }
        },
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("{e}")))
    }

    Ok(())
}

pub fn collectstatic(sitecfg: &SiteConfig) -> Result<(), Error> {
    mkdir("static/pages/")?;

    for entry in WalkDir::new(&sitecfg.pagedir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            match entry.path().to_string_lossy().strip_prefix(&sitecfg.pagedir) {
                Some(p) => fs::create_dir_all(format!("static/pages/{}", p))?,
                None => fs::create_dir_all(format!("static/pages/{}", entry.path().to_string_lossy()))?
            }
        }
        
        if entry.path().is_file() {
            match entry.path().extension() {
                Some(fp) => match sitecfg.filetypes.get(&fp.to_string_lossy().into_owned()) {
                    Some(f) => match f {
                        ExtensionFileType::Config => continue,
                        _ => match entry.path().to_string_lossy().strip_prefix(&sitecfg.pagedir) {
                            Some(p) => fs::copy(entry.path(), format!("static/pages/{}", p)).unwrap(),
                            None => fs::copy(entry.path(), format!("static/pages/{}", entry.path().to_string_lossy())).unwrap()
                        }
                    }
                    None => continue
                },
                None => continue
            };
        };
    }

    match fs::read_dir(&sitecfg.staticdir) {
        Ok(d) => {
            for entry in d {
                match entry {
                    Ok(rd) => match fs::copy(rd.path(), format!("static/{}", rd.path().to_string_lossy().strip_prefix(&sitecfg.staticdir).unwrap())) {
                        Ok(_) => (),
                        Err(ce) => return Err(Error::new(ErrorKind::Other, format!("collectstatic failed to copy {} to static/{}, {}", rd.path().to_string_lossy(), rd.path().to_string_lossy(), ce)))
                    }
                    Err(re) => return Err(Error::new(ErrorKind::Other, format!("collectstatic: {re}")))
                }
            }
        },
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("collectstatic: {e}")))
    }

    Ok(())
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

pub fn mkdir(path: &str) -> Result<(), Error> {
    match std::fs::create_dir(path) {
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {
                info!("loadfonts: directory {path} already exists, continuing");
                Ok(())
            }
            _ => {
                Err(Error::new(ErrorKind::Other, format!("Error creating directory {path}: {e}")))
            }
        }
        Ok(_) => {
            info!("{path} directory created");
            Ok(())
        }
    }
}

pub fn read_file<T: AsRef<Path>>(path: T) -> Result<String, Error> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(ErrorKind::NotFound, format!("File not found: {}", path.as_ref().to_string_lossy()))),
            // Change from v1: do not panic; return an error instead
            _ => return Err(Error::new(ErrorKind::Other, format!("Error reading from file {}: {e}", path.as_ref().to_string_lossy())))
        }
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;

    Ok(buff)
}

pub fn write_file<T: AsRef<Path>>(path: T, data: &str) -> Result<(), Error> {
    
    fs::write(path, data)?;

    Ok(())
}

pub fn mdpath2html(path: &str, headerids: bool) -> Result<String, Error> {
    let mut comrak_options = Options::default();
    comrak_options.render.unsafe_ = true;
    comrak_options.extension.table = true;
    if headerids {
        comrak_options.extension.header_ids = Some("".to_string());
    }
    let markdown = match read_file(path) {
        Ok(c) => c,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to render markdown file {path}: {e}")))
    };
    let content = markdown_to_html(&markdown, &comrak_options);

    Ok(content)
}

// path arg is used to append the path to the content file
pub fn loadcontent(pagemap: &HashMap<String, Page>, page: &Page, path: &str, tmpl: &tera::Tera) -> Result<IndexMap<String, Content>, Error> {
    let mut pagecontent: IndexMap<String, Content> = IndexMap::new();

    for (id, content) in page.content.clone().into_iter() {
        
        let newcontent = match content {
            Content::Linklist(c) => {
                let mut ctx = Context::new();
                ctx.insert("pages", pagemap);
                
                ctx.insert("links", &c.links);
                let rendered = match tmpl.render("types/linklist.html", &ctx) {
                    Ok(c) => c,
                    Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error when rendering page {path}: {:?}", e)))
                };

                let mut newcontent = c.clone();
                newcontent.rendered = rendered;

                Content::Linklist(newcontent)
            },
            Content::Content(c) => {
                let mut ctx = Context::new();
                // &path should already end with "/" so there should not be another "/" between the path and file name
                let content = match mdpath2html(&format!("{}{}", &path, &c.content), page.headerids) {
                    Ok(c) => c,
                    Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error when rendering page {path}: {e}")))
                };

                ctx.insert("content", &content);

                let rendered = match tmpl.render("types/content.html", &ctx) {
                    Ok(c) => c,
                    Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error when rendering page {path}: {e}")))
                };

                let mut newcontent = c.clone();
                newcontent.rendered = rendered;

                Content::Content(newcontent)
            }
        };

        pagecontent.insert(id.to_owned(), newcontent);
    }

    Ok(pagecontent)
}

pub fn loadpages(sitecfg: &SiteConfig) -> Result<HashMap<String, Page>, Error> {
    let mut pagemap: HashMap<String, Page> = HashMap::new();

    let tmpl = Tera::new(&format!("{}/**/*.html", sitecfg.templatedir)).unwrap();
    
    for entry in WalkDir::new(&sitecfg.pagedir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            let pagejson = format!("{}/page.json", entry.path().to_string_lossy());
            let pagepath = format!("{}/", &entry.path().to_string_lossy());
            let pagejsonpath = Path::new(&pagejson);
            if pagejsonpath.exists() {
                if pagejsonpath.is_file() {
                    let pageconfigraw = &read_file(&pagejson).unwrap();
                    match serde_json::from_str::<Page>(pageconfigraw) {
                        Ok(pageconfig) => {
                            let mut pagecfg = pageconfig.clone();

                            if pagecfg.favicon.is_empty() {
                                pagecfg.favicon = format!("/static/favicons/default_{}.ico", pagecfg.theme)
                            }
                            pagemap.insert(pagepath.strip_prefix(&sitecfg.pagedir).unwrap().to_string(), pagecfg);
                        }
                        Err(err) => {
                            warn!("loadpages: {pagejson} failed to deserialize: {err}");
                            continue;
                        }
                    };
                } else {
                    warn!("loadpages: page.json in {} is a directory", entry.path().to_string_lossy());
                }
            } else {
                warn!("loadpages: page.json not found in {}", entry.path().to_string_lossy());
            }
        }
    }

    // Catch-22: Pages must be loaded to load pages in order to fill in linklist entries with information of a page
    let mut newpagemap: HashMap<String, Page> = HashMap::new();
    for (pagepath, pagecfg) in pagemap.iter() {
        let mut newpagecfg: Page = pagecfg.clone();

        newpagecfg.content = match loadcontent(&pagemap, pagecfg, &format!("{}{}", &sitecfg.pagedir, pagepath), &tmpl) {
            Ok(c) => c,
            Err(e) => {
                warn!("loadpages: {e}");
                continue    
            }
        };

        newpagemap.insert(pagepath.to_string(), newpagecfg);
    }

    Ok(newpagemap)
}

pub fn loadconfig() -> Result<SiteConfig, Error> {
    let mut siteconfig: SiteConfig = serde_json::from_str(&read_file("ctclsite-config/config.json").unwrap()).unwrap();

    mkdir("static/")?;

    siteconfig.fonts = match loadfonts(&siteconfig) {
        Ok(t) => t,
        Err(e) => return Err(e)
    }; 

    // Themes must be loaded before pages are loaded
    siteconfig.themes = match loadthemes(&siteconfig) {
        Ok(t) => t,
        Err(e) => return Err(e)
    };

    mkfavicons(&siteconfig.themes)?;
    match collectstatic(&siteconfig) {
        Ok(_) => (),
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("collectstatic: {e}")))
    };

    // Catch-22: Pages must be loaded to load pages in order to fill in linklist entries with information of a page
    siteconfig.pages = loadpages(&siteconfig)?;
    
    if siteconfig.pages.is_empty() {
        error!("No pages found");
        return Err(Error::new(ErrorKind::NotFound, "No pages found"));
    }
    if siteconfig.themes.is_empty() {
        error!("No themes found");
        return Err(Error::new(ErrorKind::NotFound, "No themes found"));
    }

    Ok(siteconfig)
}