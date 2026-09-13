use std::{fs, path::PathBuf};

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

    pub fn path(&self) -> anyhow::Result<PathBuf> {
        let config_dir = get_config_dir()?;
        return Ok(config_dir.join("config.json"));
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(self.path()?, contents)?;
        Ok(())
    }
}

pub fn base(notes_dir: Vec<&str>) -> Config {
    Config {
        notes: notes_dir.into_iter().map(String::from).collect(),
        exclude: vec![String::from(".*")],
    }
}
