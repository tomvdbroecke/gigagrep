// Uses
use crate::utils::{format_line, get_mode, line_to_check, read_file, search_string};
use crate::{Args, Mode};
use anyhow::{Context, Error};
use colored::Colorize;
use log::debug;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::Write;
use std::io::{BufWriter, Stdout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fs, thread};
use std::{io, path::PathBuf};

// Struct for message parsing
struct SearchResult {
    order: usize,
    content: Option<String>,
    filepath: String,
}

// Process command function
pub(crate) fn process_command(args: &Args) -> Result<(), Error> {
    debug!("processing command");

    // Check if path is a file or directory
    let mode = get_mode(&args.path)?;

    // Prepare stdout for writing to cli
    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout);

    // Start channels for sending/receiving messages
    let (tx, rx) = mpsc::channel::<SearchResult>();

    // Start consumer thread
    let consumer = start_consumer_thread(Arc::new(Mutex::new(rx)), Arc::new(Mutex::new(handle)));

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
                let stdout = io::stdout();
                let mut handle = io::BufWriter::new(stdout);
                writeln!(handle, "{}", &args.path.red().bold())?;
            }
            process_file(
                args,
                &PathBuf::from(&args.path),
                &tx,
                Arc::new(Mutex::new(0)),
            )
        }
        Mode::Directory => {
            let mut order = Arc::new(Mutex::new(0));
            process_directory(args, &tx, order)
        }
    }?;

    // Drop transmission channel
    drop(tx);

    // Wait for consumer thread to finish
    consumer.join();

    // Return Ok()
    Ok(())
}

// @todo dont forget about: if args.verbose.log_level().is_some() { main
// Process file function
fn process_file(
    args: &Args,
    filepath: &PathBuf,
    tx: &Sender<SearchResult>,
    order: Arc<Mutex<usize>>,
) -> Result<(), Error> {
    // Retrieve lines from file
    let lines =
        read_file(filepath).with_context(|| format!("could not read file '{}'", &args.path))?;

    // Setup the search string
    let search_string = search_string(&args.exact_match, &args.case_insensitive, &args.pattern);

    // Order logic
    let mut ord = order.lock().unwrap();
    let original_order = ord.clone();

    // Loop over lines
    for (line_number, line) in lines.enumerate() {
        let line = line?;
        let line_to_check = line_to_check(&args.case_insensitive, &line);

        if line_to_check.contains(&search_string) {
            if *ord == original_order {
                *ord += 1;
            }
            tx.send(SearchResult {
                order: original_order,
                content: format_line(args, &line, &line_number),
                filepath: filepath.to_string_lossy().into_owned(),
            })?;
        };
    }

    // Return OK
    Ok(())
}

// Process directory function
fn process_directory(
    args: &Args,
    tx: &Sender<SearchResult>,
    order: Arc<Mutex<usize>>,
) -> Result<(), Error> {
    let path = PathBuf::from(&args.path);
    process_directory_contents(args, &path, tx, order)
}

// Recursive directory processing function
fn process_directory_contents(
    args: &Args,
    path: &PathBuf,
    tx: &Sender<SearchResult>,
    order: Arc<Mutex<usize>>,
) -> Result<(), Error> {
    // Read the contents of the directory
    let entries =
        fs::read_dir(path).with_context(|| format!("Could not read directory '{:?}'", path))?;

    // Iterate over the directory entries
    entries.par_bridge().for_each_with(tx.clone(), |s, entry| {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                // Search (recursive)
                if args.recursive {
                    process_directory_contents(args, &entry.path(), s, order.clone());
                }
            } else {
                // Process the file
                process_file(args, &entry.path(), tx, order.clone());
            }
        }
    });

    Ok(())
}

// Consumer thread function
// @bug: this doesnt work yet, its counting into the next order before all inputs have been printed
// (I'm sure it's not received more results fast enough)
fn start_consumer_thread(
    rx: Arc<Mutex<Receiver<SearchResult>>>,
    handle: Arc<Mutex<BufWriter<io::Stdout>>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer: HashMap<usize, SearchResult> = HashMap::new();
        let mut next_order = 0;

        loop {
            // Attempt to receive a result
            let result = {
                let rx = rx.lock().unwrap();
                rx.recv().ok()
            };

            match result {
                Some(result) => {
                    // If the order of the result matches the next expected order, process it
                    // directly
                    if result.order == next_order {
                        print_result(&result, &handle);
                        next_order += 1;
                    } else {
                        // If the result arrives out of order, store it in the buffer
                        buffer.insert(result.order, result);
                    }

                    // Process any buffered results that are now in order
                    while let Some(result) = buffer.remove(&next_order) {
                        print_result(&result, &handle);
                        next_order += 1;
                    }
                }
                None => break,
            }
        }
    })
}

// Helper to print result
fn print_result(result: &SearchResult, handle: &Arc<Mutex<BufWriter<io::Stdout>>>) {
    if let Some(content) = &result.content {
        let mut handle = handle.lock().unwrap();
        writeln!(handle, "{}", content).expect("Failed to write to buffer");
    }
}
