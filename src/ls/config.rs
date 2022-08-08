use std::path::PathBuf;

use super::file::{File, Directory, LsError};

/// Represents the configuration for this command.
/// We can use string slices because the config lives and dies with the command line
/// arguments in the main function.
pub struct Config {
    pub file_vec: Vec<File>,
    pub dir_vec: Vec<Directory>,
    pub error_vec: Vec<LsError>,
    pub long_list: bool,
    pub list_all: bool,
}

impl Config {
    /// Constructs a default config object where all options are off.
    fn default() -> Config {
        Config {
            file_vec: vec![],
            dir_vec: vec![],
            error_vec: vec![],
            long_list: false,
            list_all: false,
        }
    }

    /// Scans command line arguments and returns a configuration option with the 
    /// corresponding options set and paths added. 
    /// Returns an error only if a specified option does not exist.
    pub fn parse_args(args: &[String]) -> Config {
        let mut config = Config::default();

        for arg in &args[1..] {
            let bytes = arg.as_bytes();
            // Options consist of a '-' followed by at least one character.
            // Anything else should be treated as a path name.
            if bytes.len() > 1 && bytes[0] == b'-' {
                for &byte in &bytes[1..] {
                    config.set_option(byte);
                }
            } else {
                config.collect_file(arg);
            }
        }

        if config.file_vec.is_empty() && config.dir_vec.is_empty() && 
            config.error_vec.is_empty() {
            config.collect_file(".");
        }

        config
    }


    pub fn set_option(&mut self, byte: u8) {
        match byte {
            b'a'  => self.list_all = true,
            _ => println!("option not recognized"),
        };
    }

    fn collect_file(&mut self, path_name: &str) {
        let path_name = PathBuf::from(path_name);
        if path_name.is_file() {
            self.file_vec.push(File { 
                file_str: path_name,
            });
        } else if path_name.is_dir() {
            let dir_iter = path_name.read_dir().unwrap()
                .map(|dir_entry| PathBuf::from(dir_entry.unwrap().file_name()));
            self.dir_vec.push(Directory {
                dir_str: path_name,
                file_name_vec: dir_iter.collect(),
            });
        } else {
            self.error_vec.push(LsError {
                error_str: 
                    format!("could not find file or directory: {}", path_name.display()),
            });
        }
    }

    pub fn print_errors(&self) {
        for error in self.error_vec.iter() {
            println!("{}", error.error_str);
        }
    }

    pub fn print_files(&self) {
        for file in self.file_vec.iter() {
            println!("{}", file.file_str.display());
        }
    }

    pub fn print_dirs(&self) {
        for dir in self.dir_vec.iter() {
            println!("{}:", dir.dir_str.display());
            for file_name in dir.file_name_vec.iter() {
                println!("{}", file_name.display());
            }
        }
    }
}


