use anyhow::{Result, bail};
use std::env;

mod config;
mod schedule;
mod store;
mod utils;

fn main() -> Result<()> {
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
        ["help"] => {
            print_help();
            Ok(())
        }
        ["run"] => daily_run(),
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
    s.update_notes()?;

    Ok(())
}

fn print_help() {
    println!(
        r#"zettel-merken is your daily review helper

Usage:

  zettel-merken init <notes_dir>    Initial setup
  zettel-merken config              Opens config file
  zettel-merken help                Show this help
  zettel-merken run                 Daily review run"#
    );
}

fn daily_run() -> Result<()> {
    let cfg = config::read()?;
    let mut st = store::new(cfg)?;
    st.update_notes()?;
    st.up_for_review()?;
    Ok(())
}
