// Uses
use log::debug;
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
