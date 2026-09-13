use std::env;

mod config;
mod utils;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] => anyhow::bail!("needs a command"),
        ["init", notes_dir @ ..] => init(notes_dir.to_vec()),
        ["run"] => Ok(()),
        ["config"] => Ok(()),
        ["help"] => Ok(()),
        [_, ..] => anyhow::bail!("unknown command"),
    }
}

fn init(notes_dir: Vec<&str>) -> anyhow::Result<()> {
    let c = config::default(notes_dir);
    c.load();
    c.write("/tmp/config.json")?;
    Ok(())
}
