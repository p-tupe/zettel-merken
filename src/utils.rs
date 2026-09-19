use crate::config;
use anyhow::Result;
use globset::Glob;
use std::ops::Add;
use std::{env, path::PathBuf};
use time::{UtcDateTime, ext::NumericalDuration};
use walkdir::{DirEntry, WalkDir};

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
        gb.add(Glob::new(e)?);
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

pub fn local_notification(title: &str, subtitle: &str) -> Result<()> {
    use mac_notification_sys::*;

    let bundle = get_bundle_identifier_or_default("tips");
    set_application(&bundle).unwrap();
    send_notification(title, None, subtitle, None).unwrap();

    Ok(())
}

pub fn get_next_review_date(
    prev_reviews: Vec<String>,
    next_review: Option<String>,
) -> (Vec<String>, Option<String>) {
    let count: u32 = prev_reviews.len().try_into().unwrap();
    let count = if count == 10 {
        10 // maximum
    } else if next_review.is_some() {
        count + 1 // std incr
    } else {
        0 // First time
    };

    let now = UtcDateTime::now();
    let mut new_prev_reviews = Vec::from(prev_reviews);
    new_prev_reviews.push(now.to_string());
    let new_next_review = Some(now.add(i64::pow(2, count).days()).to_string());

    return (new_prev_reviews, new_next_review);
}
