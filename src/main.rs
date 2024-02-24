use std::{fs::File, io::{self, BufRead, BufReader, Lines}, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: PathBuf
}

fn main() {
    let args: Args = Args::parse();
    
    if let Ok(lines) = read_file(&args.path) {
        for line in lines.flatten() {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
    }
}

fn read_file(filepath: &PathBuf) -> io::Result<Lines<BufReader<File>>> {
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);

    Ok(reader.lines())
}
