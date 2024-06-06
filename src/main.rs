use clap::{command, Parser};
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(name = "roundup")]
#[command(about = "Utility to count lines of code in a project")]
struct Args {
    dir: String,
    filetype: Option<String>,
}

/*
CLI-based lines-of-code analyser. E.g.,
    `roundup` for every type of file
    `roundup rs` for Rust
*/
fn main() {
    let args = Args::parse();

    println!(
        "Running on directory: {}. Filetype: {}",
        args.dir,
        args.filetype.unwrap_or("all".to_string()),
    );

    let data_result = File::open("test.py");

    // Reading a file returns a Result enum
    // Result can be a file or an error
    let mut file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    // Look at walkdir crate for file handling https://docs.rs/walkdir/latest/walkdir/
}
