use std::{
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::utils::get_config_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    notes: Vec<String>,
    exclude: Vec<String>,
}

impl Config {
    pub fn load(&self) {
        print!("Config loaded {:?}!", self)
    }

    pub fn write(&self) -> anyhow::Result<()> {
        Ok(fs::write(path()?, serde_json::to_string_pretty(self)?)?)
    }
}

pub fn path() -> anyhow::Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

pub fn base(notes_dir: Vec<&str>) -> Config {
    Config {
        notes: notes_dir.into_iter().map(String::from).collect(),
        exclude: vec![String::from(".*")],
    }
}

pub fn read() -> anyhow::Result<Config> {
    Ok(serde_json::from_reader(File::open(path()?)?)?)
}
