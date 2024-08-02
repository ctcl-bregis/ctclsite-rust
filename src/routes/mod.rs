// ctclsite-rust - CTCL 2020-2024
// File: src/routes/mod.rs
// Purpose: Routes module
// Created: February 26, 2024
// Modified: August 2, 2024

mod about;
mod blog;
mod linklist;
mod logger;
mod projects;
mod services;
mod fileviewer;

pub use about::*;
pub use blog::*;
pub use linklist::*;
pub use logger::*;
pub use projects::*;
pub use services::*;
pub use fileviewer::*;