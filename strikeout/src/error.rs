use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Can not parse as a episode")]
    EpisodeNotFound,
    #[error("Can not find a file extension")]
    ExtensionNotFound,
    #[error("Not a valid path.")]
    InvalidPath,
    #[error("Io error.")]
    IoError(#[from] io::Error),
    #[error("Cache Parse Failed.")]
    InvalidCache(#[from] serde_json::Error),
    #[error("Not a valid Unicode File name.")]
    InvalidUnicodeFilename,
}
