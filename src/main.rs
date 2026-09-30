#![doc = include_str!("../README.md")]

use anyhow::{Result, bail};
use std::env;

use crate::utils::local_notification;

mod config;
mod store;
mod utils;

fn main() -> Result<()> {
    #[cfg(target_os = "macos")]
    notify_rust::set_application("com.apple.Terminal").unwrap();

    let args: Vec<String> = env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] => bail!("needs a command"),
        ["init"] => bail!("needs a note directory"),
        ["init", notes_dir @ ..] => init(notes_dir.to_vec()),
        ["config"] => config::edit(),
        ["help"] => print_help(),
        ["run"] => daily_run(),
        ["test"] => local_notification("test", "this is a test notif"),
        [_, ..] => bail!("unknown command"),
    }
}

fn init(notes_dir: Vec<&str>) -> Result<()> {
    config::ensure_dir()?;

    let mut cfg = config::Config::default();
    cfg.notes.extend(notes_dir.iter().map(|&s| String::from(s)));
    cfg.write()?;

    let mut s = store::new(cfg)?;
    s.migrate()?;
    s.sync()?;

    Ok(())
}

fn print_help() -> Result<()> {
    println!(
        r#"zettel-merken is your daily review helper

Usage:

  zettel-merken init <notes_dir>    Initial setup
  zettel-merken config              Opens config file
  zettel-merken help                Show this help
  zettel-merken run                 Daily review run"#
    );

    Ok(())
}

fn daily_run() -> Result<()> {
    let cfg = config::read()?;
    let mut st = store::new(cfg)?;
    st.sync()?;
    st.review()?;
    Ok(())
}
