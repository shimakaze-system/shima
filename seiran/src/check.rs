use crate::meta;
use colored::Colorize;
use md5::Digest;
use std::{
    fs, io,
    io::{Seek, SeekFrom},
};
pub fn check_md5_sum(mut file: fs::File, meta: &meta::Meta) -> anyhow::Result<bool> {
    print!("Check {}...", meta.name().cyan());
    let mut hasher = md5::Md5::new();
    file.seek(SeekFrom::Start(0))?;
    io::copy(&mut file, &mut hasher)?;
    let bin_md5 = hasher.finalize();
    let literally_md5 = base64::encode(bin_md5);
    let res = literally_md5 == meta.md5_hash;
    println!("{}", if res { "OK".green() } else { "Failed".red() });
    Ok(res)
}
