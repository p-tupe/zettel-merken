use std::{
    env, fs,
    path::{self, PathBuf},
};

use anyhow::{Context, Ok};

pub fn get_config_dir() -> anyhow::Result<PathBuf> {
    if let Some(base) = env::var_os("XDG_CONFIG_HOME") {
        let base = PathBuf::from(base);
        if base.is_absolute() {
            return Ok(base.join("zettelmerken"));
        }
    }

    if let Some(home) = env::var_os("HOME") {
        let home = PathBuf::from(home);
        return Ok(home.join(".config").join("zettelmerken"));
    }

    if let Some(dir) = dirs::config_dir() {
        return Ok(dir.join("zettelmerken"));
    }

    anyhow::bail!("could not find a config dir")
}
