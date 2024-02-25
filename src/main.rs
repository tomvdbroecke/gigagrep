use std::{fs::File, io::{self, BufRead, BufReader, Lines, Write}, path::PathBuf};
use clap::{error::ErrorKind, CommandFactory, Parser};
use anyhow::{Context, Result};
use clap_verbosity_flag::Verbosity;
use log::{debug, info};

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,
    #[command(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    if let Some(log_level) = args.verbose.log_level() {
        simple_logger::init_with_level(log_level)?;
        info!("starting with logging verbosity {}", &log_level);
    }

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
    debug!("processing command");
    let filepath: &PathBuf = &PathBuf::from(&args.path);

    let lines = read_file(filepath)
        .with_context(|| format!("could not read file '{}'", &args.path))?;

    let stdout: io::Stdout = io::stdout();
    let mut handle: io::BufWriter<io::Stdout> = io::BufWriter::new(stdout);
    
    for line in lines.flatten() {
        if line.contains(&args.pattern) {
            debug!("line containing '{}' found", &args.pattern);
            writeln!(handle, "{}", line)?;
        }
    }

    Ok(())
}

fn read_file(filepath: &PathBuf) -> Result<Lines<BufReader<File>>, std::io::Error> {
    debug!("reading file");
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);

    Ok(reader.lines())
}
