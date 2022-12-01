use actix_web::{App, web, get, error, Error, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use once_cell::sync::Lazy;
extern crate comrak;
use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};


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

#[get("/")]
async fn root() -> impl Responder {
    let mut context = Context::new();
    let title = String::from("Welcome - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("main_content.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}

async fn rl_main() -> impl Responder {
    let mut context = Context::new();
    let title = String::from("RAMList Menu - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("ramlist_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}


async fn blog_main() -> impl Responder {
    let mut context = Context::new();
    let title = String::from("Blog Posts - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("main_blog_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

async fn blog_post() -> impl Responder {
    let mut context = Context::new();
    let title = String::from("Blog Posts - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    match TEMPLATES.render("main_blog_post.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
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
            .route("/blog/", web::get().to(blog_main))
            .route("/blog/{post}/", web::get().to(blog_post))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
} 
