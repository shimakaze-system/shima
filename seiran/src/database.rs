use crate::meta::MetaTable;
use anyhow::Result;
use std::{borrow::Cow, fs, path};

const DB: &str = "data.json";

pub fn load<'a>(data_dir: Cow<'a, path::Path>) -> Result<MetaTable> {
    let db_path = data_dir.join(DB);
    let db = fs::File::open(db_path)?;
    Ok(serde_json::from_reader(&db)?)
}

pub fn save<'a>(data_dir: Cow<'a, path::Path>, data: Cow<'a, MetaTable>) -> Result<()> {
    fs::create_dir_all(&data_dir).ok();
    let db_path = data_dir.join(DB);
    let db = fs::File::with_options()
        .truncate(true)
        .write(true)
        .create(true)
        .open(db_path)?;
    serde_json::to_writer_pretty(db, &data)?;
    Ok(())
}
