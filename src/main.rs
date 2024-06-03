use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(name = "roundup")]
#[command(about = "Utility to count lines of code in a project")]
struct Args {
    dir: String,
    filetype: Option<String>,
}

fn main() {
    /*
    CLI-based lines-of-code analyser. E.g.,
        `roundup` for every type of file
        `roundup rs` for Rust
    */

    let args = Args::parse();

    println!("Directory: {}", args.dir);

    if let Some(arg2) = args.filetype {
        println!("Filetype: {}", arg2);
    } else {
        println!("Filetype: Not provided");
    }
}
