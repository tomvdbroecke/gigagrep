// Uses
use crate::{Args, Mode};
use anyhow::{Context, Error};
use colored::Colorize;
use log::debug;
use regex::Regex;
use std::fs;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

// Read file function
pub(crate) fn read_file(filepath: &PathBuf) -> Result<Lines<BufReader<File>>, std::io::Error> {
    debug!("reading file");

    // Setup reader from filepath
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);

    // Return lines
    Ok(reader.lines())
}

// Line to check based on case sensitivity
pub(crate) fn line_to_check(case_insensitive: &bool, line: &String) -> String {
    if *case_insensitive {
        line.to_lowercase()
    } else {
        line.to_string()
    }
}

// Search string based on exact match and case sensitivity
pub(crate) fn search_string(exact_match: &bool, case_insensitive: &bool, pattern: &str) -> String {
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

// Format line function
pub(crate) fn format_line(args: &Args, line: &String, line_number: &usize) -> Option<String> {
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
    if args.hide_line_numbers {
        Some(formatted_line)
    } else {
        Some(format!(
            "{}:{}",
            line_number.to_string().bold().blue(),
            formatted_line
        ))
    }
}

// Get mode (file or directory)
pub(crate) fn get_mode(path: &str) -> Result<Mode, Error> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("Could not read metadata for path '{}'", path))?;

    match (metadata.is_file(), metadata.is_dir()) {
        (true, false) => Ok(Mode::File),
        (false, true) => Ok(Mode::Directory),
        _ => Err(anyhow::anyhow!("Passed path is neither file nor directory")),
    }
}
