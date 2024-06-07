use clap::{command, Parser};
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

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

    let filetype = args.filetype.unwrap_or("all".to_string());

    println!(
        "Running on directory: {}. Filetype: {}",
        args.dir, &filetype
    );

    match walkdir(&args.dir, &filetype) {
        Ok(()) => println!("Directory walk succeeded"),
        Err(e) => println!("Directory walk failed with error: {}", e),
    }
}

fn walkdir(path: &str, file_type: &str) -> Result<(), walkdir::Error> {
    let extension = String::from(".") + file_type;

    for entry in WalkDir::new(path) {
        let entry = entry?;

        let is_file_type = entry
            .file_name()
            .to_str()
            .map(|s| s.ends_with(&extension))
            .unwrap_or(false);

        if is_file_type {
            println!("{}", entry.path().display());
            match count_lines(&entry.path().display().to_string()) {
                Ok(line_count) => println!("Lines: {}", line_count),
                Err(e) => panic!("Failed to count lines: {}", e),
            }
        }
    }
    Ok(())
}

fn count_lines(path: &str) -> Result<usize, std::io::Error> {
    let reader = BufReader::new(File::open(path)?);
    // Add 1 because last line not counted due to no termination
    Ok(reader.lines().count() + 1)
}
