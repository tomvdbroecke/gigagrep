// Uses
use crate::read_file::read_file;
use crate::Args;
use anyhow::Context;
use log::debug;
use std::{
    io::{self, BufWriter, Stdout, Write},
    path::PathBuf,
};

// Process command function
pub(crate) fn process_command(args: &Args) -> Result<(), anyhow::Error> {
    debug!("processing command");

    // Get filepath from argument
    let filepath: &PathBuf = &PathBuf::from(&args.path);

    // Retrieve lines from file
    let lines =
        read_file(filepath).with_context(|| format!("could not read file '{}'", &args.path))?;

    // Prepare stdout for writing to cli
    let stdout: io::Stdout = io::stdout();
    let mut handle: io::BufWriter<io::Stdout> = io::BufWriter::new(stdout);

    // Setup the search string
    let mut search_string: String = args.pattern.clone();
    if args.exact_match {
        search_string = format!(" {} ", args.pattern);
    }

    // Loop through the lines, if the line contains the pattern, print it to the stdout buffer
    for line in lines.map_while(Result::ok) {
        if line.contains(&search_string) {
            write_line(&mut handle, &args.pattern, &line)?;
        }
    }

    // Return OK
    Ok(())
}

// Write line function
fn write_line(
    handle: &mut BufWriter<Stdout>,
    pattern: &str,
    line: &String,
) -> Result<(), anyhow::Error> {
    debug!("line containing '{}' found", pattern);
    writeln!(handle, "{}", line)?;
    Ok(())
}
