#![feature(bool_to_option)]
mod config;
mod database;
mod download;
pub mod meta;

const APPLICATION: &str = "seiran";

pub use config::Config;
