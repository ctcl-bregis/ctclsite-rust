use actix_web::{App, web, get, error, Error, HttpResponse, HttpServer};
use tera::{Tera, Context};
use once_cell::sync::OnceCell;
use once_cell::sync::Lazy;

pub static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Template parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec!["html", ".sql"]);
    tera
});

#[get("/")]
async fn root() -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let title = String::from("CTCL Website");
    context.insert("title", &title);
    match TEMPLATES.render("main_content.html", &context) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}

#[get("/")]
async fn root() -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let title = String::from("CTCL Website");
    context.insert("title", &title);
    match TEMPLATES.render("main_content.html", &context) {
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
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
        
            
