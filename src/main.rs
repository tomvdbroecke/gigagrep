use std::{fs::File, io::{BufRead, BufReader, Error, Lines}, path::PathBuf};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String
}

/*
 @todo:
    1 ->    Make sure error formats are the same (from clap and anyhow for example)
            I think we can use this resource: https://www.rustadventure.dev/introducing-clap/clap-v4/reporting-errors-via-clap
    2 ->    Add logging
*/

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let filepath = &PathBuf::from(&args.path);
    let lines = read_file(filepath)
        .with_context(|| format!("could not read file '{}'", &args.path))?;
    
    // @todo: calling printLn very often in a loop can be slow, use a BufWriter
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
