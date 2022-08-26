use std::error::Error;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::path::PathBuf;

/// The Files struct is used to categorize files by type.
pub struct FileManager {
    pub regular_files: Vec<PathBuf>,
    pub directories: Vec<PathBuf>,
}

impl FileManager {
    pub fn collect_files(args: &[String]) -> FileManager {
        let mut regular_files = vec![];
        let mut directories = vec![];

        for path_name in args {

            
        }

        FileManager {
            regular_files,
            directories,
        }
    }

    pub fn print_regular_files(&self) {
    }

    pub fn print_directories(&self) -> Result<(), Box<dyn Error>> {
        
        Ok(())
    }
}
