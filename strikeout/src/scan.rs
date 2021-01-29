use crate::error::{Error, Result};
use std::{collections::HashSet, env::current_dir, fs, path};
use walkdir::{DirEntry, WalkDir};

const APPLICATION: &str = "strikeout";

fn data_file() -> path::PathBuf {
    let cur = current_dir().expect("can not parse current dir");
    let cur = cur.to_string_lossy().to_string();
    let name = cur.replace(path::MAIN_SEPARATOR, "_");
    let mut data_dir = dirs::data_dir().expect("system error");
    data_dir.push(APPLICATION);
    // set dir
    fs::create_dir_all(&data_dir).ok();
    data_dir.push(name);
    data_dir
}

pub fn scan_new_file(dir: &path::Path, file_list: &mut HashSet<String>) -> Vec<DirEntry> {
    let mut new_file_list = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_entry(|e| !is_hidden(e)) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = check_file(entry, file_list, &mut new_file_list) {
                log::error!("file {} process failed because of {}", file_name, e);
            }
        } else {
            log::error!("can not access file.");
        }
    }
    new_file_list
}

fn check_file(entry: DirEntry, file_list: &mut HashSet<String>, new_file_list: &mut Vec<DirEntry>) -> Result<()> {
    if !entry.file_type().is_file() {
        return Ok(());
    }
    let path = entry.path().to_str().ok_or(Error::InvalidPath)?;
    if file_list.insert(path.to_owned()) {
        new_file_list.push(entry);
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

pub fn get_file_list() -> Result<HashSet<String>> {
    let file = fs::File::open(data_file())?;
    let file_list = serde_json::from_reader(file)?;
    Ok(file_list)
}

pub fn store_file_list(old_file_list: &HashSet<String>) -> Result<()> {
    let file = fs::File::with_options()
        .truncate(true)
        .write(true)
        .create(true)
        .open(data_file())?;
    serde_json::to_writer(file, old_file_list)?;
    Ok(())
    // pass
}
