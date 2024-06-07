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

struct RoundupOutput {
    lines: u64,
    files: u32,
}

/*
CLI-based lines-of-code analyser. E.g.,
    `roundup` for every type of file
    `roundup rs` for Rust
*/
fn main() {
    let args = Args::parse();
    let filetype = args.filetype.unwrap_or("all".to_string());

    println!("Searching {} for .{} files", args.dir, &filetype);

    match walkdir(&args.dir, &filetype) {
        Ok(output) => println!("{} lines in {} files", output.lines, output.files),
        Err(e) => println!("Directory walk failed: {}", e),
    }
}

fn walkdir(path: &str, file_type: &str) -> Result<RoundupOutput, walkdir::Error> {
    let extension = String::from(".") + file_type;
    let mut total_line_count: u64 = 0;
    let mut file_count: u32 = 0;

    for entry in WalkDir::new(path) {
        let entry = entry?;

        let is_file_type = entry
            .file_name()
            .to_str()
            .map(|s| s.ends_with(&extension))
            .unwrap_or(false);

        if is_file_type {
            let a = match count_lines(&entry.path().display().to_string()) {
                Ok(line_count) => line_count,
                Err(e) => panic!("Failed to count lines: {}", e),
            };
            total_line_count += a as u64;
            file_count += 1;
        }
    }
    Ok(RoundupOutput {
        lines: total_line_count,
        files: file_count,
    })
}

fn count_lines(path: &str) -> Result<usize, std::io::Error> {
    let reader = BufReader::new(File::open(path)?);
    Ok(reader.lines().count() + 0)
}
