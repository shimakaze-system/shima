use seiran::{database, meta, Config};

async fn run(config: Config<'static>) -> anyhow::Result<()> {
    let data_dir = config.data_dir();
    let cache_dir = config.cache_dir();
    let install_dir = config.install_dir();
    let db = database::load(data_dir.clone()).unwrap_or_default();
    let uri = config.list_api();
    let data = meta::fetch(uri.as_ref()).await?;
    let delta = data.clone().into_owned() - db.into_owned();
    for meta in delta.into_iter() {
        // download(meta, cache_dir);
    }
    // check checksum
    // move(cache_dir, install_dir);
    database::save(data_dir.clone(), data)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let config = Config::from_file(Config::default_config_path())?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run(config))?;
    Ok(())
}
