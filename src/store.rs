use std::path::PathBuf;

use crate::{config, utils::get_config_dir};

use anyhow::Result;

#[derive(Debug)]
pub struct Store {
    // cfg: config::Config,
    // db: db
}

impl Store {
    pub fn update_notes(&self) -> Result<()> {
        print!("Updating notes");
        Ok(())
    }
}

pub fn create(_: config::Config) -> Result<Store> {
    // TODO: create db and tables here
    print!("Creating store at {:?}", path()?);
    Ok(Store {})
}

pub fn path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("zettelmerken.db"))
}
