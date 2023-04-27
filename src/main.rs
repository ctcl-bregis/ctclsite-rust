// Purpose: Main program code
// Date: November 28, 2022 - April 27, 2023
// CTCL - Brayden Regis - 2022-2023
use std::{error::Error, collections::BTreeMap};
use actix_web::{App, web, get, error, HttpResponse, HttpServer, http::StatusCode, Responder, web::Path, middleware::Logger};
use actix_files as fs;
use tera::Tera;
use once_cell::sync::Lazy;
use ctclsite::{csv2im, md2html, mkcontext, rl_list_gen};
use env_logger::Env;
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

// RAMList main menu
async fn rl_main() -> impl Responder {
    let mut context = mkcontext("ramlist", "root").unwrap().0;
    
    let menu = csv2im("./config/ramlist/menu.csv").unwrap();
    context.insert("menu", &menu);
    
    match TEMPLATES.render("rl_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap().clone());
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// RAMList non-menu pages
async fn ramlist(list: Path<Info>) -> impl Responder {
    let mut lists = Vec::new();
    // Only add to "lists" what is a list page
    for entry in csv2im("./config/ramlist/menu.csv").unwrap() {
        if entry["type"] == "list" {
            lists.push(entry["name"].clone());
        }
    }
    
    // Other content page: about
    if list.page == "about" {
        let result = mkcontext("ramlist", "about").unwrap();
        let mut context = result.0;
        let index = result.1;
        
        let content = md2html(&index["content"]).unwrap();
        
        context.insert("content", &content);

        match TEMPLATES.render("rl_about.html", &context) {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err.source().unwrap());
                Err(error::ErrorInternalServerError(err))
            },
        }
    // Other content page: announcements
    } else if list.page == "log" {
        let mut context = mkcontext("ramlist", "log").unwrap().0;
        
        let entries = csv2im("./config/ramlist/log/posts.csv").unwrap();
        
        let mut content = Vec::new();
        for entry in entries {
            let mut tmpim = BTreeMap::new();
            tmpim.insert("content", md2html(&format!("./config/ramlist/log/{}", entry["path"].clone())).unwrap());
            tmpim.insert("date", entry["date"].clone());
            content.push(tmpim); 
        }
        
        context.insert("content", &content);
        
        match TEMPLATES.render("rl_log.html", &context) {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err.source().unwrap());
                Err(error::ErrorInternalServerError(err))
            },
        }
    // List pages
    // Test if the page is defined in menu.csv
    } else if lists.contains(&list.page) {
        let mut context = mkcontext("ramlist", &list.page).unwrap().0;
        
        let content = rl_list_gen(&list.page).unwrap();
        let tables = &content.0;
        let headers = &content.1;
        
        context.insert("tables", &tables);
        context.insert("headers", &headers);
        context.insert("entries", &content.2);
        
        // Get vec of table keys; vendor names
        let mut tables_keys = Vec::new();
        for (key, _value) in tables.iter() {
            tables_keys.push(key);
        }
        context.insert("tables_keys", &tables_keys);
        
        // Get vec of header keys
        let mut headers_keys = Vec::new();
        for (key, _value) in headers.iter() {
            headers_keys.push(key);
        }
        context.insert("headers_keys", &headers_keys);
        
        // Width may be defined by an external config file later on
        context.insert("table_width", "2750pt");
        
        match TEMPLATES.render("rl_list.html", &context) {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err.source().unwrap());
                Err(error::ErrorInternalServerError(err))
            },
        }
    } else {
        let context = mkcontext("error", "404").unwrap().0;
        
        match TEMPLATES.render("err_404.html", &context) {
            Ok(body) => Ok(HttpResponse::build(StatusCode::NOT_FOUND).content_type("text/html; charset=utf-8").body(body)),
            Err(err) => {
                eprintln!("Tera error: {}", err.source().unwrap());
                Err(error::ErrorInternalServerError(err))
            }, 
        }
    }
}
    
// Blog post list
async fn blog_main() -> impl Responder {
    let result = mkcontext("blog", "root").unwrap();
    let mut context = result.0;
    let posts = &csv2im("./config/blog/index.csv").unwrap();
    
    context.insert("posts", posts);
    
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
    let result = mkcontext("blog", &post.page).unwrap();
    let mut context = result.0;
    let index = result.1;
    
    context.insert("content", &md2html(&format!("./config/blog/{}/content.md", &index["content"])).unwrap());
    
    match TEMPLATES.render("blog_post.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap());
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// Project list
async fn project_main() -> impl Responder {
    let result = mkcontext("projects", "root").unwrap();
    let mut context = result.0;
    
    context.insert("pages", &csv2im("./config/projects/index.csv").unwrap());
    
    match TEMPLATES.render("projects_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap());
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// Project page
async fn project_page(page: Path<Info>) -> impl Responder {    
    let result = mkcontext("projects", &page.page).unwrap();
    let mut context = result.0;
    let index = result.1;
    
    context.insert("content", &md2html(&format!("./config/projects/{}/content.md", &index["content"])).unwrap());
    
    match TEMPLATES.render("projects_page.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap());
            Err(error::ErrorInternalServerError(err))
        }, 
    } 
}

// Routes
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(|| {
         App::new().service(root)
            .wrap(Logger::default())
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            // TODO: figure out how to redirect to page with / at the end
            .service(fs::Files::new("/static/", "./static/")
                .use_last_modified(true))
            .route("/ramlist/", web::get().to(rl_main))
            .route("/ramlist/{page}/", web::get().to(ramlist))
            .route("/blog/", web::get().to(blog_main))
            .route("/blog/{page}/", web::get().to(blog_post))
            .route("/projects/", web::get().to(project_main))
            .route("/projects/{page}/", web::get().to(project_page))
            .route("/privacy/", web::get().to(privacy))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
} 
