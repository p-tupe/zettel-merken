use std::path::PathBuf;

use crate::{
    config,
    utils::{get_config_dir, get_note_entries},
};

use anyhow::Result;

#[derive(Debug)]
pub struct Store {
    // db: db
}

impl Store {
    pub fn update_notes(&self, cfg: config::Config) -> Result<()> {
        print!("Updating notes");
        for e in get_note_entries(cfg)? {
            print!("{:?}", e);
        }
        Ok(())
    }
}

pub fn create() -> Result<Store> {
    // TODO: create db and tables here
    print!("Creating store at {:?}", path()?);
    Ok(Store {})
}

pub fn path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("zettelmerken.db"))
}

pub fn connect() -> Result<Store> {
    Ok(Store {})
}
