#![feature(bool_to_option, with_options)]
mod config;
pub mod database;
mod download;
pub mod meta;

const APPLICATION: &str = "seiran";

pub use config::Config;
pub use download::download;
