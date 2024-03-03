// Modules
mod process_command;
mod utils;

// Uses
use anyhow::Result;
use clap::{error::ErrorKind, CommandFactory, Parser};
use clap_verbosity_flag::Verbosity;
use log::info;
use process_command::process_command;

// @todo
// - Allow output to file (optional, default = no)
// - Show summary of amount of lines found (optional, default = no)
// - Allow piped output
// - Do we also show filename when only searching a file?
// - Add flag for not showing filename
// - Do we want an extra whitespace after filename?

// Struct for arguments
#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Whether to match the pattern exactly"
    )]
    exact_match: bool,
    #[arg(short, long, default_value_t = false, help = "Ignores capitalization")]
    case_insensitive: bool,
    #[arg(
        short = 'l',
        long,
        default_value_t = false,
        help = "Hides line numbers"
    )]
    hide_line_numbers: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Removes the pattern highlight"
    )]
    no_pattern_highlight: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Recursively search directories"
    )]
    recursive: bool,
    #[command(flatten)]
    verbose: Verbosity,
}

// Mode
enum Mode {
    File,
    Directory,
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
    if let Err(error) = process_command(&args) {
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
