// ctclsite-rust
// CrazyblocksTechnologies Computer Laboratories 2022-2023
use std::{error::Error, collections::BTreeMap};
use actix_web::{App, web, get, error, HttpResponse, HttpServer, http::StatusCode, Responder, web::Path};
use actix_files as fs;
use tera::Tera;
use once_cell::sync::Lazy;
use ctclsite::{csv2im, md2html, mkcontext, rl_list_gen};
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
    dbg!(&sections_index);
    
    // Sections would be vector of BTreeMap's 
    let mut sections: Vec<BTreeMap<String, String>> = Vec::new();

    for i in sections_index {
        let mut section = BTreeMap::new();
        
        let bgimg = &i["bgimg"];
        
        // Insert bgimg into section if empty or not
        section.insert("bgimg", bgimg);
    
        let content = &i["content"];
        if !(content == "") {
            section.insert("content", &md2html(&format!("./config/about/main/{}", &content)).unwrap());
        } else {
            section.insert("content", &String::from(""));
        }
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

// RAMList main menu
async fn rl_main() -> impl Responder {
    let mut context = mkcontext("ramlist", "root").unwrap().0;
    
    let menu = csv2im("./config/ramlist/menu.csv").unwrap();
    context.insert("menu", &menu);
    
    match TEMPLATES.render("rl_menu.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("Tera error: {}", err.source().unwrap());
            Err(error::ErrorInternalServerError(err))
        }, 
    }
}

// RAMList List page; contents page
async fn rl_list(list: Path<Info>) -> impl Responder {
    let mut lists = Vec::new();
    // Only add to "lists" what is a list page
    for entry in csv2im("./config/ramlist/menu.csv").unwrap() {
        if entry["type"] == "list" {
            lists.push(entry["name"].clone());
        }
    }
    
    // Other content page: about
    if list.page == "about" {
        let context = mkcontext("ramlist", "about").unwrap().0;
        
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
        context.insert("entries", &entries);
        
        for entry in entries {
            let mut tmpim = BTreeMap::new();
            tmpim.insert("path", md2html(&format!("./config/ramlist/log/{}", entry["path"].clone())));
            tmpim.insert("date", Ok(entry["date"].clone()));
        }
        
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
    let mut context = mkcontext("blog", "root").unwrap().0;
    
    context.insert("posts", &csv2im("./config/blog/posts.csv").unwrap());
    
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
    let mut context = mkcontext("blog", &post.page).unwrap().0;
    
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
            .route("/ramlist/", web::get().to(rl_main))
            .route("/ramlist/{page}/", web::get().to(rl_list))
            .route("/blog/", web::get().to(blog_main))
            .route("/blog/{page}/", web::get().to(blog_post))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
} 
