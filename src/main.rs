use actix_web::{App, web, get, error, HttpResponse, HttpServer, http::StatusCode, Responder, web::Path};
use tera::{Tera, Context};
use once_cell::sync::Lazy;
use ctclsite::{csv2hm, md2html};
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
    let mut context = Context::new();
    let title = String::from("Welcome - CrazyblocksTechnologies Computer Laboratories");
    context.insert("title", &title);
    
    let content = md2html("./config/main_about.md");
    context.insert("content", &content.unwrap());
    
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
    let pagetitle = String::from("Main Menu");
    context.insert("pagetitle", &pagetitle);
    let pagedesc = String::from("Main Menu of RAMList at CrazyblocksTechnologies Computer Laboratories");
    context.insert("pagedesc", &pagedesc);
    
    let menu = csv2hm("./config/ramlist/menu.csv");
    context.insert("menu", &menu.unwrap());
    
    match TEMPLATES.render("rl_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// RAMList List page; contents page
async fn rl_list(list: Path<Info>) -> impl Responder {
    let mut context = Context::new();
    let title = String::from(format!("RAMList - CrazyblocksTechnologies Computer Laboratories"));
    context.insert("title", &title);
    
    let mut lists = Vec::new();
    
    // Other content page: about
    if list.page == "about" {
        match TEMPLATES.render("rl_about.html", &context) {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err);
                Err(error::ErrorInternalServerError(err))
            },
        }
    // Other content page: announcements
    } else if list.page == "log" {
        match TEMPLATES.render("rl_log.html", &context) {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err);
                Err(error::ErrorInternalServerError(err))
            },
        }
    // List pages
    // Test if the page is defined in menu.csv
    } else if lists.contains(&list.page) {
    // Get widths of table columns
    
    
    
        match TEMPLATES.render("rl_list.html", &context) {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err);
                Err(error::ErrorInternalServerError(err))
            },
        }
    } else {
        match TEMPLATES.render("err_404.html", &context) {
            Ok(body) => Ok(HttpResponse::build(StatusCode::NOT_FOUND).content_type("text/html; charset=utf-8").body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err);
                Err(error::ErrorInternalServerError(err))
            }, 
        }
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
            .route("/ramlist/{page}/", web::get().to(rl_list))
            .route("/blog/", web::get().to(blog_main))
            .route("/blog/{page}/", web::get().to(blog_post))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
} 
