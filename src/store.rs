use std::path::PathBuf;

use crate::{
    config::Config,
    utils::{get_config_dir, get_note_entries},
};

use anyhow::Result;
use rusqlite::{Connection, params};

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
                "insert into notes (title, path) values (?, ?)
on conflict(path) do update
set last_touched = current_timestamp;",
            )?;

            let Some(filename) = entry.file_name().to_str() else {
                continue;
            };

            let Some(pathname) = entry.path().to_str() else {
                continue;
            };

            stmt.execute(params![filename, pathname])?;
        }

        Ok(tx.commit()?)
    }

    pub fn migrate(&self) -> Result<()> {
        self.conn.execute(
            "create table if not exists notes (
        title text not null,
        path text not null unique,
        last_touched datetime default current_timestamp
    );",
            (),
        )?;

        Ok(())
    }
}

pub fn new(cfg: Config) -> Result<Store> {
    Ok(Store {
        conn: Connection::open(path()?)?,
        cfg,
    })
}

pub fn path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("zettelmerken.db"))
}
