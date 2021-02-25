use crate::meta;
use colored::Colorize;
use std::{borrow::Cow, fs, path};

pub fn install<'a>(
    meta: &meta::Meta,
    cache_dir: Cow<'a, path::Path>,
    install_dir: Cow<'a, path::Path>,
) -> anyhow::Result<()> {
    print!("Install {}...", meta.name().cyan());
    let from = cache_dir.join(meta.name());
    let to = install_dir.join(meta.name());
    fs::copy(from, to)?;
    println!("{}", "OK".green());
    Ok(())
}
