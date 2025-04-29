use std::fs;

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub blacklist: Vec<String>,
}

impl Config {
    pub fn from_default_path() -> Result<Self, Error> {
        let project_conf_dir = directories::ProjectDirs::from(
            "io",
            env!("CARGO_PKG_AUTHORS").split(",").next().unwrap(),
            env!("CARGO_PKG_NAME"),
        )
        .ok_or(Error::InitConfigFailed(
            "failed to find default config path",
        ))?;

        let config_dir = project_conf_dir.config_dir();
        fs::create_dir_all(config_dir)?;

        let conf = config_dir.join("config.toml");
        if conf.exists() {
            Ok(toml::from_str(&fs::read_to_string(conf)?)?)
        } else {
            warn!("{} not found, using default", conf.to_string_lossy());
            Ok(Self::default())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            blacklist: vec![
                "cls-magic2_x64.exe",
                "cls-magic2_x86.exe",
                "cls-magic2l_x64.exe",
                "cls-magic2l_x86.exe",
                "oo2reck.exe",
                "rz.exe",
                "xtool.exe",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        }
    }
}
