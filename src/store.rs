use std::{fs::metadata, path::PathBuf};

use time::UtcDateTime;

use crate::{
    config::Config,
    utils::{get_config_dir, get_next_review_date, get_note_entries},
};

use anyhow::Result;
use rusqlite::{Connection, config::DbConfig, params};
use serde_json::json;

#[derive(Debug)]
pub struct Store {
    conn: Connection,
    cfg: Config,
}

#[derive(Debug)]
pub struct Note {
    pub title: String,
    path: String,
    prev_reviews: Vec<String>,
    // mtime: String,
    next_review: Option<String>,
}

impl Store {
    pub fn sync(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;

        for entry in get_note_entries(&self.cfg)? {
            let mut stmt = tx.prepare(
                "insert into notes (title, path, mtime, next_review) values (?, ?, ?, ?)
on conflict(path) do update
set last_sync = current_timestamp;",
            )?;

            let Some(filename) = entry.file_name().to_str() else {
                continue;
            };

            let Some(pathname) = entry.path().to_str() else {
                continue;
            };

            let Ok(mtime) = metadata(entry.path()).and_then(|f| f.modified()) else {
                continue;
            };
            let modified: UtcDateTime = mtime.into();

            stmt.execute(params![
                filename,
                pathname,
                modified.to_string(),
                get_next_review_date(vec![], None).1
            ])?;
        }

        Ok(tx.commit()?)
    }

    pub fn review(&self) -> Result<()> {
        let mut stmt = self.conn.prepare(&format!(
            "select title, path, next_review, prev_reviews from notes where next_review > current_timestamp order by mtime limit {};",
            self.cfg.max_per_review
        ))?;

        let results = stmt.query_map([], |r| {
            let s_str: String = r.get(3)?;
            let prev_reviews: Vec<String> = serde_json::from_str(&s_str).expect("valid json");

            Ok(Note {
                title: r.get(0)?,
                path: r.get(1)?,
                next_review: r.get(2)?,
                prev_reviews,
            })
        })?;

        let mut to_review = Vec::new();
        for n in results {
            let n = n?;
            to_review.push(n.title);
            let review = get_next_review_date(n.prev_reviews, n.next_review);
            self.conn.execute(
                "update notes set next_review = (?), prev_reviews = (?) where path = (?);",
                params![review.1, json!(review.0).to_string(), n.path],
            )?;
        }

        self.cfg.notify(to_review)
    }

    pub fn migrate(&self) -> Result<()> {
        self.conn.execute(
            "create table if not exists notes (
        title text not null,
        path text not null unique,
        mtime text not null, -- file's modification time
        last_sync datetime default current_timestamp, -- sweep during daily run
        prev_reviews jsonb default '[]', -- json array of timestamps
        next_review datetime
    );",
            (),
        )?;

        Ok(())
    }
}

pub fn new(cfg: Config) -> Result<Store> {
    let conn = Connection::open(path()?)?;
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
    Ok(Store { conn, cfg })
}

pub fn path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("zettelmerken.db"))
}
