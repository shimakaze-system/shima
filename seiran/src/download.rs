use crate::meta;
use colored::*;
use futures_util::StreamExt;
use std::{borrow::Cow, fs, io, io::prelude::*, os::unix::fs::PermissionsExt, path};

pub async fn download(target: &meta::Meta, desc: Cow<'_, path::Path>) -> anyhow::Result<fs::File> {
    // create cache dir
    fs::create_dir_all(desc.clone()).ok();
    let name = target.name();
    print!("Downloading {}...", name.cyan());
    io::stdout().flush().unwrap();
    let res = reqwest::get(&target.media_link).await?;
    let desc = desc.join(&name);

    if res.status().is_success() {
        let mut stream = res.bytes_stream();
        let mut file = fs::File::with_options()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open(desc)?;
        file.set_permissions(fs::Permissions::from_mode(0o755))?;
        while let Some(bytes) = stream.next().await {
            file.write_all(&bytes?)?;
        }
        file.sync_all()?;
        println!("{}", "OK".green());
        Ok(file)
    } else {
        Err(anyhow::Error::msg("Error fetching file!"))
    }
}
