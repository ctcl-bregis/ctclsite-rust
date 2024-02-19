// ctclsite-rust - CTCL 2020-2024
// File: runner_dev
// Purpose: Main code
// Created: November 28, 2022
// Modified: February 19, 2024

#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}