use std::{fs::File, io::{BufRead, BufReader, Lines}, path::PathBuf};
use clap::{error::ErrorKind, CommandFactory, Parser};
use anyhow::{Context, Result};

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String
}

/*
 @todo:
    1 ->    Add logging
*/

fn main() -> Result<()> {
    let args: Args = Args::parse();

    if let Err(error) = process_command(args) {
        let mut cmd = Args::command();
        if let Some(source) = error.source() {
            cmd.error(ErrorKind::Io, format!("{}\n\ncause: {}", &error, source)).exit()
        } else {
            cmd.error(ErrorKind::Io, format!("{}", &error)).exit()
        }
    }

    Ok(())
}

fn process_command(args: Args) -> Result<(), anyhow::Error> {
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

fn read_file(filepath: &PathBuf) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);

    Ok(reader.lines())
}
