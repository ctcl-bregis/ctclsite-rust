// ctclsite-rust - CTCL 2022-2024
// File: src/routes/mod.rs
// Purpose: Routes module
// Created: February 26, 2024
// Modified: March 4, 2024

mod about;
mod bcc_tc;
mod blog;
mod logger;
mod projects;
mod services;

pub use about::*;
pub use bcc_tc::*;
pub use blog::*;
pub use logger::*;
pub use projects::*;
pub use services::*;
