use clap::Parser;


// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    // The pattern to look for
    pub pattern: String,
    // The path to the file to read
    pub path: std::path::PathBuf,

    // Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


#[test]
fn test_find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

/*
    Since stdout expects bytes (not strings),
    we use std::io::Write instead of std::fmt::Write.
    As a result, we give an empty vector as “writer”
    in our tests (its type will be inferred to Vec<u8>),
    in the assert_eq! we use a b"foo".
    (The b prefix makes this a byte string literal
    so its type is going to be &[u8] instead of &str).
 */
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}
/*
    this function is used to simulate a loading bar progress
*/
pub fn display_load_indicatior(/* callback to execute */) -> Result<(), Box<dyn std::error::Error>> {
    let pb = indicatif::ProgressBar::new(100);
    for _ in 0..100 {
        // pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }

    pb.finish_with_message("done");

    Ok(())
}
