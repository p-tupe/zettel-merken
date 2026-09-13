use std::{fs, path::PathBuf};

use anyhow::{Result, bail};

use serde::{Deserialize, Serialize};

use crate::utils::get_config_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    notes: Vec<String>,
    exclude: Vec<String>,
}

pub fn base(notes_dir: Vec<&str>) -> Config {
    Config {
        notes: notes_dir.into_iter().map(String::from).collect(),
        exclude: vec![String::from(".*")],
    }
}

impl Config {
    pub fn write(&self) -> Result<()> {
        if fs::exists(path()?)? {
            bail!("config already exists");
        }
        Ok(fs::write(path()?, serde_json::to_string_pretty(self)?)?)
    }
}

pub fn ensure_dir() -> Result<()> {
    Ok(fs::create_dir_all(get_config_dir()?)?)
}

pub fn path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

pub fn read() -> Result<Config> {
    Ok(serde_json::from_reader(fs::File::open(path()?)?)?)
}
