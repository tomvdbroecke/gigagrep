// Modules
mod process_command;
mod read_file;

// Uses
use anyhow::Result;
use clap::{error::ErrorKind, CommandFactory, Parser};
use clap_verbosity_flag::Verbosity;
use log::info;
use process_command::process_command;

// @todo
// - Allow output to file (optional, default = no)
// - When -q flag is passed (verbose), dont print to stdout (but still print to file)
// - Show line numbers (optional, default = yes)
// - Show amount of lines found (optional, default = yes)
// - Add loading bar
// - Make found part bold (optional, defauls = yes)
// - Allow piped output
// - Allow searching through directory

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
