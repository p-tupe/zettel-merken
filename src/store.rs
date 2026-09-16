use std::path::PathBuf;

use crate::{
    config::Config,
    utils::{get_config_dir, get_note_entries},
};

use anyhow::Result;
use rusqlite::{Connection, params_from_iter};

#[derive(Debug)]
pub struct Store {
    conn: Connection,
    cfg: Config,
}

impl Store {
    pub fn update_notes(&self) -> Result<()> {
        let mut stmt = String::from("insert into notes (title, path) values ");
        let mut entries = vec![];
        for e in get_note_entries(self.cfg.clone())? {
            if let Some(title) = e.file_name() {
                if let Some(title) = title.to_str().map(String::from) {
                    if let Some(path) = e.as_path().to_str().map(String::from) {
                        stmt.push_str("(?, ?),");
                        entries.push(title);
                        entries.push(path);
                    }
                }
            }
        }
        if let Some(stmt) = stmt.strip_suffix(",") {
            self.conn.execute(stmt, params_from_iter(entries))?;
        }
        Ok(())
    }

    pub fn migrate(&self) -> Result<()> {
        self.conn.execute(
            "create table if not exists notes (
        title text not null,
        path text not null,
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
