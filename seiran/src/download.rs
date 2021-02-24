use crate::meta;
use colored::*;
use futures_util::StreamExt;
use std::{borrow::Cow, fs, io::prelude::*, path};

pub async fn download<'a>(target: &meta::Meta, desc: Cow<'a, path::Path>) -> anyhow::Result<()> {
    // create cache dir
    fs::create_dir_all(desc.clone()).ok();
    let name = target.name();
    let res = reqwest::get(&target.media_link).await?;
    let desc = desc.join(&name);

    if res.status().is_success() {
        print!("Downloading {}...", name.cyan());
        let mut stream = res.bytes_stream();
        let mut file = fs::File::create(desc)?;
        while let Some(bytes) = stream.next().await {
            file.write_all(&bytes?)?;
        }
        println!("{}", "OK".green());
    } else {
        return Err(anyhow::Error::msg("Error fetching file!"));
    }
    Ok(())
}
