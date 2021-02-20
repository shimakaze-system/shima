use seiran::{database, meta, Config};

async fn run(config: Config<'static>) -> anyhow::Result<()> {
    let uri = config.list_api();
    let data = meta::fetch(uri.as_ref()).await?;
    dbg!(data);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let config = Config::from_file(Config::default_path())?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run(config))?;
    Ok(())
}
