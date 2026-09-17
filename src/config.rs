use std::{env, fs, path::PathBuf, process::Command};

use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::utils::get_config_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: i8,
    pub notes: Vec<String>,
    pub exclude: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            notes: vec![],
            exclude: vec![String::from(".*")],
        }
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

pub fn edit() -> Result<()> {
    let path = path()?;

    let Some(editor) = env::var_os("VISUAL").or_else(|| env::var_os("EDITOR")) else {
        println!("{}", path.display());
        return Ok(());
    };

    let _ = Command::new(editor).arg(path).status()?;
    Ok(())
}
