use crate::utils::{get_config_dir, local_notification};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf, process::Command};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Notification {
    Local,
    Mail { id: String },       // todo: https://docs.rs/lettre/latest/lettre/
    Slack { channel: String }, // todo: https://docs.rs/slack-hook/latest/slack_hook/
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: i8,
    pub notes: Vec<String>,
    pub exclude: Vec<String>,
    pub max_per_review: i8,
    pub notification: Notification,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            notes: vec![],
            exclude: vec![String::from(".*")],
            max_per_review: 5,
            notification: Notification::Local,
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

    // find a better place later
    pub fn notify(&self, notes: Vec<String>) -> Result<()> {
        use Notification::*;

        let summary = if notes.len() > 0 {
            &notes.join(", ")
        } else {
            "all good today!"
        };

        match self.notification {
            Local => local_notification("Zettel Merken Daily Review List", summary),
            _ => bail!("only local notification implemented"),
        }
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
