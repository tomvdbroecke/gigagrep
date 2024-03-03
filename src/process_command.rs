// Uses
use crate::read_file::read_file;
use crate::Args;
use anyhow::Context;
use colored::Colorize;
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
    let search_string = search_string(&args.exact_match, &args.pattern);

    // Loop through the lines, if the line contains the pattern, print it to the stdout buffer
    let mut line_number: u64 = 1;
    for line in lines.map_while(Result::ok) {
        let line_to_check = line_to_check(&args.case_insensitive, &line);

        if line_to_check.contains(&search_string) {
            write_line(&mut handle, args, &line, &line_number)?;
        }

        line_number += 1;
    }

    // Return OK
    Ok(())
}

// Line to check based on case sensitivity
fn line_to_check(case_insensitive: &bool, line: &String) -> String {
    if *case_insensitive {
        line.to_lowercase()
    } else {
        line.to_string()
    }
}

// Search string based on exact match
fn search_string(exact_match: &bool, pattern: &str) -> String {
    if *exact_match {
        format!(" {} ", &pattern)
    } else {
        pattern.to_string()
    }
}

// Write line function
fn write_line(
    handle: &mut BufWriter<Stdout>,
    args: &Args,
    line: &String,
    line_number: &u64,
) -> Result<(), anyhow::Error> {
    debug!("line containing '{}' found", args.pattern);
    if args.hide_line_numbers {
        writeln!(handle, "{}", line)?;
    } else {
        writeln!(handle, "{}{}", format!("{}:", line_number).blue(), line)?;
    }
    Ok(())
}
