mod unix;

use std::error::Error;
use std::path::Path;

use crate::unix::config::Config as UnixConfig;
use crate::unix::file_manager::FileManager as UnixFileManager;

pub fn run_for_unix(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {

    let config = UnixConfig::from_args(args);

    config.print_regular_files();
    config.print_directories()?;

    Ok(())
}
