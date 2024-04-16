// ctclsite-rust - CTCL 2022-2024
// File: src/routes/projects/mod.rs
// Purpose: Projects module
// Created: March 1, 2024
// Modified: April 10, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use indexmap::IndexMap;
use tera::Context;
use crate::{mdpath2html, PageType, SiteCfg, Section};

fn mkcontext(sitecfg: &SiteCfg, subpage: &str) -> Context {
    let mut ctx = Context::new();

    if subpage == "root" {
        ctx.insert("menu", &sitecfg.projectscfg.cats);
        
        let rootpage = &sitecfg.projectscfg.root;
        ctx.insert("title", &rootpage.title);
        let defaultfavicon = &format!("/static/favicons/default_{}.ico", &rootpage.theme);
        ctx.insert("favicon", match &rootpage.favicon {
            Some(link) => link,
            None => defaultfavicon
        });
        ctx.insert("themename", &rootpage.theme);
        ctx.insert("themecolor", &sitecfg.themes.get(&rootpage.theme).unwrap().color);
        ctx.insert("desc", &rootpage.desc);
    } else {
        // The page should be known to exist at this point
        let projectscfg = &sitecfg.projectscfg.clone().getpages();
        let subpagetype = projectscfg.get(subpage).unwrap();
    
        match subpagetype {
            PageType::PageTypeSections(subpage) => {
                ctx.insert("title", &subpage.title);
                let defaultfavicon = &format!("/static/favicons/default_{}.ico", &subpage.theme);
                ctx.insert("favicon", match &subpage.favicon {
                    Some(link) => link,
                    None => defaultfavicon
                });
                ctx.insert("themename", &subpage.theme);
                ctx.insert("themecolor", &sitecfg.themes.get(&subpage.theme).unwrap().color);
                ctx.insert("desc", &subpage.desc);
                ctx.insert("sectionpixfont", &subpage.sectionpixfont);

                let mut renderedsections: IndexMap<String, Section> = IndexMap::new();
                let mut isvideo: bool = false;
                for section in subpage.sections.iter() {
                    let mut renderedsection = section.1.clone();
    
                    renderedsection.content = mdpath2html(&renderedsection.content, false).unwrap();
                    
                    /* Show the button and load vids.js if any of the sections have video backgrounds */
                    if renderedsection.bgvid.is_some() {
                        isvideo = true;
                    }

                    if renderedsection.fitscreen.is_none() {
                        renderedsection.fitscreen = Some(true);
                    }

                    renderedsections.insert(section.0.to_string(), renderedsection);
                }

                ctx.insert("video", &isvideo);
                ctx.insert("sections", &renderedsections);
            }
            PageType::PageTypeContent(subpage) => {
                ctx.insert("title", &subpage.title);
                let defaultfavicon = &format!("/static/favicons/default_{}.ico", &subpage.theme);
                ctx.insert("favicon", match &subpage.favicon {
                    Some(link) => link,
                    None => defaultfavicon
                });
                ctx.insert("themename", &subpage.theme);
                ctx.insert("themecolor", &sitecfg.themes.get(&subpage.theme).unwrap().color);
                ctx.insert("desc", &subpage.desc);

                ctx.insert("rendered", &mdpath2html(&subpage.content, true).unwrap());
            }
        }
    }
    ctx
}

pub async fn projects_index(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let ctx = mkcontext(sitecfg.get_ref(), "root");
    
    let s = match tmpl.render("projects_menu.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}

pub async fn projects_page(page: web::Path<String>, tmpl: web::Data<tera::Tera>, sitecfg: web::Data<SiteCfg>) -> Result<impl Responder, Error> {
    let projectscfg = sitecfg.projectscfg.clone().getpages();

    if !projectscfg.contains_key(&page.clone()) {
        return Ok(HttpResponse::NotFound().body("Page not found"))
    }

    let ctx = mkcontext(sitecfg.get_ref(), &page);

    let subpagecfg = projectscfg.get(&page.clone()).unwrap();

    let template = match subpagecfg {
        PageType::PageTypeContent(_) => "content.html",
        PageType::PageTypeSections(_) => "sections.html"
    };
    
    let s = match tmpl.render(template, &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}