// Uses
use crate::utils::{get_mode, line_to_check, read_file, search_string, write_line};
use crate::{Args, Mode};
use anyhow::{Context, Error};
use colored::Colorize;
use log::debug;
use std::fs;
use std::io::Write;
use std::io::{BufWriter, Stdout};
use std::{io, path::PathBuf};

// Process command function
pub(crate) fn process_command(args: &Args) -> Result<(), Error> {
    debug!("processing command");

    // Check if path is a file or directory
    let mode = get_mode(&args.path)?;

    // Prepare stdout for writing to cli
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    // If mode is only one file, process it, otherwise, process the directory
    match mode {
        Mode::File => {
            // Check if recursive flag is set
            if args.recursive {
                return Err(anyhow::anyhow!(
                    "Recursive flag is set but path points to a file"
                ));
            }
            if !args.hide_filepath {
                writeln!(handle, "{}", &args.path.red().bold())?;
            }
            process_file(args, &PathBuf::from(&args.path), &mut handle)
        }
        Mode::Directory => process_directory(args, &mut handle),
    }
}

// Process file function
fn process_file(
    args: &Args,
    filepath: &PathBuf,
    handle: &mut BufWriter<Stdout>,
) -> Result<(), Error> {
    // Retrieve lines from file
    let lines =
        read_file(filepath).with_context(|| format!("could not read file '{}'", &args.path))?;

    // Setup the search string
    let search_string = search_string(&args.exact_match, &args.case_insensitive, &args.pattern);

    // Loop through the lines, if the line contains the pattern, print it to the stdout buffer
    let mut line_number: u64 = 1;
    for line in lines.map_while(Result::ok) {
        let line_to_check = line_to_check(&args.case_insensitive, &line);

        if line_to_check.contains(&search_string) {
            // @todo: first time this happens, print filename
            write_line(handle, args, &line, &line_number)?;
        }

        line_number += 1;
    }

    // Return OK
    Ok(())
}

// Process directory function
fn process_directory(args: &Args, handle: &mut BufWriter<Stdout>) -> Result<(), Error> {
    let path = PathBuf::from(&args.path);
    process_directory_contents(args, &path, handle)
}

// Recursive directory processing function
fn process_directory_contents(
    args: &Args,
    path: &PathBuf,
    handle: &mut BufWriter<Stdout>,
) -> Result<(), Error> {
    // Read the contents of the directory
    let entries =
        fs::read_dir(path).with_context(|| format!("Could not read directory '{:?}'", path))?;

    // Iterate over the directory entries
    let mut count = 0;
    for entry in entries {
        let entry =
            entry.with_context(|| format!("Error reading directory entry in '{:?}'", path))?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // If recursive flag is set, recursively process subdirectories
            if args.recursive {
                process_directory_contents(args, &entry_path, handle)?;
            }
        } else {
            // Process files using process_file function
            if !args.hide_filepath {
                if count > 0 {
                    writeln!(handle)?;
                }
                writeln!(handle, "{}", &entry_path.to_string_lossy().red().bold())?;
            }
            process_file(args, &entry_path, handle)?;
            count += 1;
        }
    }

    Ok(())
}
