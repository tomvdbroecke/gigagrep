use std::{fs::File, io::{BufRead, BufReader, Error, Lines}, path::PathBuf};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Args {
    pattern: String,
    path: PathBuf
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let lines = read_file(&args.path)
        .with_context(|| format!("could not read file {:?}", &args.path))?;
    
    for line in lines.flatten() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn read_file(filepath: &PathBuf) -> Result<Lines<BufReader<File>>, Error> {
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);

    Ok(reader.lines())
}
