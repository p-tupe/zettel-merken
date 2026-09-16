use std::{env, path::PathBuf};

use globset::Glob;
use walkdir::{DirEntry, WalkDir};

use crate::config;

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

pub fn get_note_entries(cfg: &config::Config) -> anyhow::Result<Vec<DirEntry>> {
    let mut gb = globset::GlobSetBuilder::new();
    for e in &cfg.exclude {
        gb.add(Glob::new(&e)?);
    }
    let excl = gb.build()?;

    let mut entries = vec![];
    for dir in &cfg.notes {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| e.depth() == 0 || !excl.is_match(e.path()))
        {
            let entry = entry?;
            if entry.file_type().is_file() {
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}
