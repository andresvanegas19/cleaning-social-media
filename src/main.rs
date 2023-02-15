use log::{info, debug};
use env_logger::Env;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use clap::Parser;

mod libs;

fn read_file(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let file_reading = File::open(path)?;
    let mut reader = BufReader::with_capacity(10, file_reading);

    assert!(reader.buffer().is_empty());

    if reader.fill_buf()?.len() > 0 {
        assert!(!reader.buffer().is_empty());
    }

    Ok(reader.expect("Can't read Cargo.toml"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let args = libs::Cli::parse();

    info!("already read the content of the file");
    // manage the information of the file
    let content = read_file(&args.path)?;
    // let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()));
    libs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    libs::display_load_indicatior()?;

    debug!("Run without problems!");
    Ok(())
}


