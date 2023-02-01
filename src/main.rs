// Purpose: Main program code
// Date: November 28, 2022 - February 1, 2023
// CrazyblocksTechnologies Computer Laboratories, Brayden Regis - 2022-2023
use std::{error::Error, collections::BTreeMap};
use actix_web::{App, web, get, error, HttpResponse, HttpServer, http::StatusCode, Responder, web::Path};
use actix_files as fs;
use tera::Tera;
use once_cell::sync::Lazy;
use ctclsite::{csv2im, md2html, mkcontext};
#[macro_use] extern crate serde_derive;

#[derive(Deserialize)]
struct Info {
    page: String,
}

pub static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Template parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);
    tera
});

// About
#[get("/")]
async fn root() -> impl Responder {
    let mkcontext_res = mkcontext("about", "root").unwrap();
    // mkcontext result 0 is the context
    let mut context = mkcontext_res.0;
    
    // Get section info
    let sections_index = csv2im("./config/about/main/sections.csv").unwrap();
    
    // Sections would be vector of BTreeMap's 
    let mut sections = Vec::new();

    for i in sections_index {
        let mut section = BTreeMap::new();
        
        // Insert bgimg into section if empty or not
        section.insert("bgimg", i["bgimg"].clone());
    
        let content = &i["content"];
        if !(content == "") {
            section.insert("content", md2html(&format!("./config/about/main/{}", &content)).unwrap());
        } else {
            section.insert("content", String::from(""));
        }
        
        sections.push(section);
    }
    
    context.insert("sections", &sections);
    
    match TEMPLATES.render("main_content.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap());
            Err(error::ErrorInternalServerError(err))
        },
    }
}

// Privacy Policy page just reads a Markdown file
async fn privacy() -> impl Responder {
    let result = mkcontext("about", "privacy").unwrap();
    let mut context = result.0;
    
    context.insert("content", &md2html("./config/about/about_privacy.md").unwrap());
    
    match TEMPLATES.render("main_privacy.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap().clone());
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}
    
// Blog post list
async fn blog_main() -> impl Responder {
    let result = mkcontext("blog", "root").unwrap();
    let mut context = result.0;
    
    context.insert("posts", &csv2im("./config/blog/index.csv").unwrap());
    
    match TEMPLATES.render("blog_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap());
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// Blog post content
async fn blog_post(post: Path<Info>) -> impl Responder {
    // TODO: Read info for context from blog post index
    let result = mkcontext("blog", &post.page).unwrap();
    let mut context = result.0;
    let index = result.1;
    
    context.insert("content", &md2html(&format!("./config/blog/{}", &index["content"])).unwrap());
    
    match TEMPLATES.render("blog_post.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap());
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
         App::new().service(root)
            // TODO: figure out how to redirect to page with / at the end
            .service(fs::Files::new("/static/", "./static/")
                .use_last_modified(true))
            .route("/blog/", web::get().to(blog_main))
            .route("/blog/{page}/", web::get().to(blog_post))
            .route("/privacy/", web::get().to(privacy))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
} 
