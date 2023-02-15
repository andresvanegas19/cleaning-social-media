use clap::Parser;


// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,

    // Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}