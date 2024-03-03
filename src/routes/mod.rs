// ctclsite-rust - CTCL 2022-2024
// File: src/routes/mod.rs
// Purpose: Routes module
// Created: February 26, 2024
// Modified: March 3, 2024

mod about;
mod bcc_cc;
mod blog;
mod logger;
mod projects;
mod services;

pub use about::*;
pub use bcc_cc::*;
pub use blog::*;
pub use logger::*;
pub use projects::*;
pub use services::*;
