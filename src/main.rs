use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: PathBuf
}

fn main() {
    let args: Args = Args::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
