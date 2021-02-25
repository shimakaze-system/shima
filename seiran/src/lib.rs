#![feature(bool_to_option, with_options)]
mod check;
mod config;
pub mod database;
mod download;
mod install;
pub mod meta;

const APPLICATION: &str = "seiran";

pub use check::check_md5_sum;
pub use config::Config;
pub use download::download;
pub use install::install;
