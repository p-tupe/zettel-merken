use std::path::PathBuf;

use crate::{
    config::Config,
    schedule::Schedule,
    utils::{get_config_dir, get_note_entries},
};

use anyhow::Result;
use rusqlite::{Connection, config::DbConfig, params};
use serde_json::json;

#[derive(Debug)]
pub struct Store {
    conn: Connection,
    cfg: Config,
}

impl Store {
    pub fn update_notes(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;

        for entry in get_note_entries(&self.cfg)? {
            let mut stmt = tx.prepare(
                "insert into notes (title, path, schedule) values (?, ?, ?)
on conflict(path) do update
set last_updated = current_timestamp;",
            )?;

            let Some(filename) = entry.file_name().to_str() else {
                continue;
            };

            let Some(pathname) = entry.path().to_str() else {
                continue;
            };

            stmt.execute(params![
                filename,
                pathname,
                json!(Schedule::new()).to_string()
            ])?;
        }

        Ok(tx.commit()?)
    }

    pub fn up_for_review(&self) -> Result<()> {
        Ok(())
    }

    pub fn migrate(&self) -> Result<()> {
        self.conn.execute(
            "create table if not exists notes (
        title text not null,
        path text not null unique,
        last_updated datetime default current_timestamp,
        schedule jsonb -- {reviews: [...], next: '', last: ''}
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
