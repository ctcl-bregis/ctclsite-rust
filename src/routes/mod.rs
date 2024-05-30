// ctclsite-rust - CTCL 2020-2024
// File: src/routes/mod.rs
// Purpose: Routes module
// Created: February 26, 2024
// Modified: April 9, 2024

mod about;
mod linklist;
mod blog;
mod logger;
mod projects;
mod services;

pub use about::*;
pub use linklist::*;
pub use blog::*;
pub use logger::*;
pub use projects::*;
pub use services::*;
