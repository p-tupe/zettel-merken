use std::{env, path::PathBuf};

pub fn get_config_dir() -> anyhow::Result<PathBuf> {
    let dir = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|b| b.is_absolute())
        .or_else(|| env::home_dir().map(|h| h.join(".config")))
        .or_else(dirs::config_dir)
        .ok_or_else(|| anyhow::anyhow!("could not find config dir"))?
        .join("zettelmerken");

    Ok(dir)
}
