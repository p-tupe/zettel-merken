use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};

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
    let path = path
        .to_str()
        .ok_or(anyhow::anyhow!("could not find config path"))?;

    Ok(env::var_os("VISUAL")
        .map_or(env::var_os("EDITOR"), Some)
        .map_or_else(
            || print!("{}", path),
            |editor| {
                Command::new(editor)
                    .arg(path)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("something went wrong");
            },
        ))
}
