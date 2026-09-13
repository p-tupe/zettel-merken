use std::{fs, path::PathBuf};

use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::utils::get_config_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub notes: Vec<String>,
    pub exclude: Vec<String>,
}

pub fn base(notes_dir: Vec<&str>) -> Config {
    Config {
        notes: notes_dir.into_iter().map(String::from).collect(),
        exclude: vec![String::from(".*")],
    }
}

impl Config {
    pub fn write(&self) -> Result<()> {
        let f = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path()?)?;

        Ok(serde_json::to_writer_pretty(f, self)?)
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
