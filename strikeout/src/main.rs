use clap::Clap;
use std::{
    env,
    path::{Path, PathBuf},
};
use strikeout::{link, scan};

#[derive(Clap)]
#[clap(version=clap::crate_version!(), author=clap::crate_authors!())]
struct Opts {
    /// source dir
    src: PathBuf,
    /// dest dir
    dest: PathBuf,
    /// test run without link
    #[clap(short, long = "dry-run")]
    dry_run: bool,
    /// set working dir
    #[clap(short, long = "working-dir")]
    working_dir: Option<PathBuf>,
    /// log level
    #[clap(short, long)]
    verbose: bool,
    /// update index cache only, no file link
    #[clap(short, long)]
    index: bool,
}

fn index_mode(src: &Path) {
    let mut file_set = std::collections::HashSet::new();
    scan::scan_new_file(&src, &mut file_set);
    if let Err(e) = scan::store_file_list(&file_set) {
        log::error!("file list cache failed.\n{}", e);
    };
}

fn main() {
    let Opts {
        src,
        dest,
        dry_run,
        working_dir,
        verbose,
        index,
    } = Opts::parse();
    // log set
    let log_level = if verbose {
        simplelog::LevelFilter::max()
    } else {
        simplelog::LevelFilter::Error
    };
    let log_config = simplelog::ConfigBuilder::new().set_time_format_str("%+").build();
    let term_mode = simplelog::TerminalMode::Stdout;
    let color = simplelog::ColorChoice::Auto;
    simplelog::TermLogger::init(log_level, log_config, term_mode, color).expect("log set failed");
    // panic logs
    log_panics::init();
    // change working dir
    if let Some(ref working_dir) = working_dir {
        env::set_current_dir(working_dir).expect("working dir change failed");
    }
    // index mode
    if index {
        return index_mode(&src);
    }
    // get file list
    let mut file_set = scan::get_file_list()
        .map_err(|e| {
            log::error!("data cache load failed: {}", e);
        })
        .unwrap_or_default();
    let list = scan::scan_new_file(&src, &mut file_set);
    for file in list.into_iter() {
        let target = link::map_to_dest(file.path(), &src, &dest);
        log::info!("{} -> {}", file.path().display(), target.display());
        if dry_run {
            log::info!("{} skiped.", file.path().display())
        } else if let Err(e) = link::link_to(file.path(), &target) {
            log::error!("{} link failed. {}.", file.path().display(), e);
        }
    }
    if !dry_run {
        if let Err(e) = scan::store_file_list(&file_set) {
            log::error!("file list cache failed.\n{}", e);
        };
    }
}
