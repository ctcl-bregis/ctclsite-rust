// ctclsite-rust
// CrazyblocksTechnologies Computer Laboratories 2022-2023
use std::{fs::OpenOptions, io::Write, path::Path};
use colored::{Colorize, control};
use chrono::Local;

const log_directory = "log/";
const current_log = "current.csv";

pub enum Header {
    SUCCESS,
    INFO,
    WARNING,
    ERROR
}