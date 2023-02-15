use log::{info, debug};
use env_logger::Env;

use std::fs::File;
use std::io::{ self, BufReader, BufRead};
use clap::Parser;

mod response;

/*
    this function is used to simulate a loading bar progress
*/
fn display_load_indicatior(/* callback to execute */) -> Result<(), Box<dyn std::error::Error>> {
    let pb = indicatif::ProgressBar::new(100);
    for _ in 0..100 {
        // pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }

    pb.finish_with_message("done");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    // read the file
    let args = response::Cli::parse();
    let file_reading = File::open(args.path)?;
    let mut reader = BufReader::new(file_reading);
    assert!(reader.buffer().is_empty());

    // Check the buffer is empty
    // let content = match reader {
    //     Ok(content) => { content },
    //     Err(error) => { return Err(error.into()); }
    // };

    if reader.fill_buf()?.len() > 0 {
        assert!(!reader.buffer().is_empty());
    }

    debug!("Reading the file without problems in the path: {}", args.path);


    debug!("Content of the file");
    // manage the information of the file
    for line in reader.lines() {
        let line = line?;

        info!("{}", line);
    }

    display_load_indicatior()?;

    debug!("Run without problems!");
    Ok(())
}
