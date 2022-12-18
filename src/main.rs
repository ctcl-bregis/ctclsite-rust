use actix_web::{App, web, get, error, HttpResponse, HttpServer, http::StatusCode, Responder, web::Path};
use tera::{Tera};
use once_cell::sync::Lazy;
use ctclsite::{csv2bt, md2html, mkcontext};
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
    let mut context = mkcontext("about").unwrap();
    
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
    let mut context = mkcontext("ramlist").unwrap();
    
    let menu = csv2bt("./config/ramlist/menu.csv").unwrap();
    context.insert("menu", &menu);
    
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
    let mut context = mkcontext("ramlist").unwrap();
    
    let mut lists = Vec::new();
    // Only add to "lists" what is a list page
    for entry in csv2bt("./config/ramlist/menu.csv").unwrap() {
        if entry["type"] == "list" {
            lists.push(entry["name"].clone());
        }
    }
    
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
    let mut context = mkcontext("blog").unwrap();
    
    context.insert("posts", &csv2bt("./config/blog/blog_index.csv").unwrap());
    
    match TEMPLATES.render("blog_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// Blog post content
async fn blog_post() -> impl Responder {
    // TODO: Read info for context from blog post index
    let mut context = mkcontext("blog").unwrap();
    
    match TEMPLATES.render("blog_post.html", &context) {
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
