// Modules
mod process_command;
mod read_file;

// Uses
use anyhow::Result;
use clap::{error::ErrorKind, CommandFactory, Parser};
use clap_verbosity_flag::Verbosity;
use log::info;
use process_command::process_command;

/**
 * @todo
 * - Think of cool name and rename package (gigagrep?)
 * - Restructure for testing and add some tests
 * - Add exact match flag (optional, default not)
 * - Add ignore casing flag (optional, default no)
 * - Allow output to file (optional, default not)
 * - Show line numbers (optional, default yes)
 * - Show amount of lines found (optional, default yes)
 * - Make found part bold (optional, default yes)
 * - Allow piped output?
 * - Allow searching through directory?
 */

// Struct for arguments
#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,
    #[command(flatten)]
    verbose: Verbosity,
}

// Main function
fn main() -> Result<()> {
    // Parse the arguments
    let args: Args = Args::parse();

    // Initialize logger based on log level
    if let Some(log_level) = args.verbose.log_level() {
        simple_logger::init_with_level(log_level)?;
        info!("starting with logging verbosity {}", &log_level);
    }

    // Process the command, if an error occurs, format it the same way as 'clap'
    if let Err(error) = process_command(args) {
        let mut cmd = Args::command();
        if let Some(source) = error.source() {
            cmd.error(ErrorKind::Io, format!("{}\n\ncause: {}", &error, source))
                .exit()
        }
        cmd.error(ErrorKind::Io, format!("{}", &error)).exit()
    }

    // Return OK
    Ok(())
}
