use anyhow::{Result, bail};
use std::env;

mod config;
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

        ["run"] => Ok(()),

        ["config"] => {
            let c = config::read()?;
            print!("{:?}", c);

            let s = store::connect()?;
            s.update_notes(c)?;

            Ok(())
        }

        ["help"] => Ok(()),

        [_, ..] => bail!("unknown command"),
    }
}

fn init(notes_dir: Vec<&str>) -> anyhow::Result<()> {
    let cfg = config::base(notes_dir);
    config::ensure_dir()?;
    cfg.write()?;

    let s = store::create()?;
    s.update_notes(cfg)?;

    Ok(())
}
