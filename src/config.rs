use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    notes: Vec<String>,
    exclude: Vec<String>,
}

impl Config {
    pub fn load(&self) {
        print!("Config loaded {:?}!", self)
    }

    pub fn write(&self, dir: &str) -> anyhow::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(dir, contents)?;
        Ok(())
    }
}

pub fn default(notes_dir: Vec<&str>) -> Config {
    Config {
        notes: notes_dir.into_iter().map(String::from).collect(),
        exclude: vec![String::from(".*")],
    }
}
