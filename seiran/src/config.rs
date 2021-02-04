use crate::APPLICATION;
use anyhow::Result;
use serde::Deserialize;
use std::{borrow::Cow, fs, io::Read, path};

fn cache_dir<'a>() -> Cow<'a, path::Path> {
    dirs::cache_dir().expect("No XDG_CACHE_HOME setted.").into()
}

fn data_dir<'a>() -> Cow<'a, path::Path> {
    dirs::data_dir().expect("No XDG_CACHE_HOME setted.").into()
}

#[derive(Deserialize)]
pub struct Config<'a> {
    /// https://storage.googleapis.com/storage/v1/
    api_endpoint: Cow<'a, str>,
    /// example_bucket
    bucket_name: Cow<'a, str>,
    /// default to XDG_CACHE_HOME
    #[serde(default = "cache_dir")]
    cache_dir: Cow<'a, path::Path>,
    #[serde(default = "data_dir")]
    data_dir: Cow<'a, path::Path>,
}

impl<'a> Config<'a> {
    pub fn cache_dir(&self) -> Cow<'a, path::Path> {
        self.cache_dir.join(APPLICATION).into()
    }

    pub fn data_dir(&self) -> Cow<'a, path::Path> {
        self.data_dir.join(APPLICATION).into()
    }

    pub fn list_api(self) -> Cow<'a, str> {
        self.api_endpoint + "b/" + self.bucket_name + "/o"
    }

    pub fn from_file(file: Cow<'a, path::Path>) -> Result<Self> {
        let mut file = fs::File::open(file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn default_path() -> Cow<'a, path::Path> {
        dirs::config_dir()
            .expect("No XDG_CONFIG_HOME setted.")
            .join(APPLICATION)
            .into()
    }
}
