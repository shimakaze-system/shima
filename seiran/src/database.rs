use crate::meta::MetaTable;
use anyhow::Result;
use std::{borrow::Cow, fs, path};

const DB: &str = "data.json";

pub fn load(data_dir: Cow<'_, path::Path>) -> Result<MetaTable> {
    let db_path = data_dir.join(DB);
    let db = fs::File::open(db_path)?;
    Ok(serde_json::from_reader(&db)?)
}

pub fn save(data_dir: Cow<'_, path::Path>, data: Cow<'_, MetaTable>) -> Result<()> {
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
