// ctclsite-rust - CTCL 2020-2024
// File: src/app.rs
// Purpose: Main code
// Created: November 28, 2022
// Modified: February 25, 2024

#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use tera::Tera;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}