mod files;

use crate::files::FileSummary;
use clap::{command, Parser};
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

    let filetypes: Vec<String> = parse_filetypes(args.filetype);

    println!("Searching {} for {:?} files", args.dir, &filetypes);

    match walkdir(&args.dir, filetypes) {
        Ok(output) => println!("{} lines in {} files", output.lines, output.files),
        Err(e) => println!("Directory walk failed: {}", e),
    }
}

fn parse_filetypes(filetype_arg: Option<String>) -> Vec<String> {
    match filetype_arg {
        Some(filetype_arg) => {
            if filetype_arg.contains(",") {
                filetype_arg
                    .split(",")
                    .map(|s| ".".to_string() + s)
                    .collect()
            } else {
                vec![".".to_string() + &filetype_arg]
            }
        }
        None => Vec::<String>::new(),
    }
}

fn walkdir(path: &str, desired_file_types: Vec<String>) -> Result<RoundupOutput, walkdir::Error> {
    let mut total_line_count: u64 = 0;
    let mut file_count: u32 = 0;

    for entry in WalkDir::new(path) {
        let dir_entry = entry?;

        // Don't read directories
        if dir_entry.file_type().is_dir() {
            continue;
        }

        let file_summary = FileSummary::from(dir_entry.path().display().to_string());

        let is_desired_type = match file_summary.extension {
            Some(ref ext) => desired_file_types.contains(&format!(".{}", &ext)),
            None => desired_file_types.is_empty(),
        };

        if is_desired_type {
            total_line_count += file_summary.read_num_lines().unwrap();
            file_count += 1;
        }
    }
    Ok(RoundupOutput {
        lines: total_line_count,
        files: file_count,
    })
}
