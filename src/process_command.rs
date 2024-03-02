// Uses
use crate::read_file::read_file;
use crate::Args;
use anyhow::Context;
use log::debug;
use std::{
    io::{self, Write},
    path::PathBuf,
};

// Process command function
pub(crate) fn process_command(args: Args) -> Result<(), anyhow::Error> {
    debug!("processing command");

    // Get filepath from argument
    let filepath: &PathBuf = &PathBuf::from(&args.path);

    // Retrieve lines from file
    let lines =
        read_file(filepath).with_context(|| format!("could not read file '{}'", &args.path))?;

    // Prepare stdout for writing to cli
    let stdout: io::Stdout = io::stdout();
    let mut handle: io::BufWriter<io::Stdout> = io::BufWriter::new(stdout);

    // Loop through the lines, if the line contains the pattern, print it to the stdout buffer
    for line in lines.map_while(Result::ok) {
        if line.contains(&args.pattern) {
            debug!("line containing '{}' found", &args.pattern);
            writeln!(handle, "{}", line)?;
        }
    }

    // Return OK
    Ok(())
}
