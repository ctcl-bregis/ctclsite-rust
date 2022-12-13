use actix_web::{App, web, get, error, Error, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use once_cell::sync::Lazy;
extern crate comrak;
use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};
use csv::{Reader, StringRecord};
// lib.rs
use ctclsite::readcsv;


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
    let mut context = Context::new();
    let title = String::from("Welcome - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("main_content.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}

// RAMList main menu
async fn rl_main() -> impl Responder {
    let mut context = Context::new();
    let title = String::from("RAMList Menu - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("ramlist_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// RAMList List page; contents page
async fn rl_list(list: web::Path<String>) -> impl Responder {
    let mut context = Context::new();
    let title = String::from(format!("RAMList - CrazyblocksTechnologies Computer Laboratories"));
    context.insert("title", &title);
    match TEMPLATES.render("main_blog_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}
    
// Blog post list
async fn blog_main() -> impl Responder {
    let mut context = Context::new();
    let title = String::from("Blog Posts - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("main_blog_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// Blog post content
async fn blog_post() -> impl Responder {
    let mut context = Context::new();
    let title = String::from("Blog Posts - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("main_blog_post.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(root)
            // TODO: figure out how to redirect to page with / at the end
            .route("/ramlist/", web::get().to(rl_main))
            .route("/ramlist/{list}/", web::get().to(rl_list))
            .route("/blog/", web::get().to(blog_main))
            .route("/blog/{post}/", web::get().to(blog_post))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
} 
