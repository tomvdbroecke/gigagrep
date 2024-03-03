// Uses
use crate::read_file::read_file;
use crate::Args;
use anyhow::Context;
use colored::Colorize;
use log::debug;
use regex::Regex;
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
    let search_string = search_string(&args.exact_match, &args.case_insensitive, &args.pattern);

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

// Search string based on exact match and case sensitivity
fn search_string(exact_match: &bool, case_insensitive: &bool, pattern: &str) -> String {
    let str = if *exact_match {
        format!(" {} ", &pattern)
    } else {
        pattern.to_string()
    };

    if *case_insensitive {
        str.to_lowercase()
    } else {
        str
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

    // Highlight the found pattern in the line
    let formatted_line = if !args.no_pattern_highlight {
        let pattern = &args.pattern;
        let regex = Regex::new(&format!("(?i){}", regex::escape(pattern))).unwrap(); // Case-insensitive regex
        regex
            .replace_all(line, |caps: &regex::Captures| {
                format!("{}", caps.get(0).unwrap().as_str().yellow().bold())
            })
            .to_string()
    } else {
        line.to_string()
    };

    // If not in quiet mode, print the found line (with or without line numbers)
    if args.verbose.log_level().is_some() {
        if args.hide_line_numbers {
            writeln!(handle, "{}", formatted_line)?;
        } else {
            writeln!(
                handle,
                "{}{}",
                format!("{}:", line_number).bold().blue(),
                formatted_line
            )?;
        }
    }

    Ok(())
}
