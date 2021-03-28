use colored::Colorize;
use seiran::{check_md5_sum, database, download, install, meta, Config};
use std::io::{self, Write};

fn failed(e: anyhow::Error) -> anyhow::Error {
    println!("{}\n", "Failed".red());
    e
}

async fn run(config: Config<'static>) -> anyhow::Result<()> {
    let data_dir = config.data_dir();
    let cache_dir = config.cache_dir();
    let install_dir = config.install_dir();
    println!(
        "{}\ndata dir: {}\ncache dir: {}\ninstall dir: {}",
        "::<> Check config.".blue(),
        data_dir.to_string_lossy().cyan(),
        cache_dir.to_string_lossy().cyan(),
        install_dir.to_string_lossy().cyan()
    );
    println!("{}", "::<> Seiran.".blue());
    let prev = database::load(data_dir.clone()).unwrap_or_default();
    let uri = config.list_api();
    let data = meta::fetch(uri.as_ref()).await.map_err(failed)?;
    let delta = data.clone().into_owned() - prev;
    if delta.is_empty() {
        println!("{}", "No update.".green());
    }
    for meta in delta.iter() {
        let file = download(meta, cache_dir.clone()).await.map_err(failed)?;
        if !check_md5_sum(file, &meta).map_err(failed)? {
            println!("{}", "Exited".red());
            return Err(anyhow::Error::msg("Check_sum failed."));
        }
        install(&meta, cache_dir.clone(), install_dir.clone()).map_err(failed)?;
    }
    database::save(data_dir.clone(), data)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let default_path = Config::default_config_path();
    print!("Load config from {}...", default_path.to_string_lossy().cyan());
    io::stdout().flush().unwrap();
    let config = Config::from_file(default_path).map_err(failed)?;
    println!("{}", "OK".green());
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run(config))?;
    Ok(())
}
