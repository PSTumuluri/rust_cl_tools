mod config;
mod long_list_utils;

use std::error::Error;
use std::io;
use std::fs::{self, DirEntry, ReadDir};
use std::path::{Path, PathBuf};

use crate::ls::config::Config;

/// The ls command lists files in a directory.
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {

    let config = parse_config(&args)?;
    for path in &config.path_vec {
        if let Err(_) = process_path(path, &config) {
                println!("Directory not found: {}", path.display());
        }
    }

    Ok(())
}

/// Scans command line arguments and returns a configuration option with the corresponding
/// options set and paths added.
/// Returns an error only if a specified option does not exist.
fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    let mut config = Config::from_default();

    for arg in &args[1..] {
        let bytes = arg.as_bytes();
        // Options start with '-'. Anything else should be treated as a path name.
        if bytes[0] == b'-' {
            for &byte in &bytes[1..] {
                config.set_option(byte)?;
            }
        } else {
            config.add_path(arg);
        }
    }

    if config.path_vec.is_empty() {
        config.add_path(".");
    }

    Ok(config)
}

/// Visits the specified path, printing its information if a file, or printing its 
/// contents' information if a directory.
/// Returns an error if the path does not correspond to an existing directory.
fn process_path(path: &Path, config: &Config) -> io::Result<()> {
    if path.is_file() {
        process_entry(path, config);
    } else if path.is_dir() {
        let dir_iter = path.read_dir()?;
        for entry in dir_iter {
            let entry = entry?;
            process_entry(&entry.path(), config);
        }
    } else {
        println!("File type not supported for {}", path.display());
    }

    Ok(())
}

/// Displays a directory entry according the configured settings.
/// Panics when the OsString cannot be converted to a UTF-8 string because I don't know
/// what else to do.
fn process_entry(path: &Path, config: &Config) {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if !config.list_all && file_name.as_bytes()[0] == b'.' {
        return;
    }
    if config.long_list {
        print_long_list(path);
    } else {
        println!("{}", file_name);
    }
}

fn print_long_list(path: &Path) {
    println!("{}", long_list_utils::make_long_list_string(path));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_adds_directory_paths_and_sets_options() {
        let args: Vec<String> = vec![String::from("ls"), String::from("."), 
            String::from("-l"), String::from(".."), String::from("-a"), 
            String::from("~")];
        let config = parse_config(&args).unwrap();

        assert!(config.long_list);
        assert!(config.list_all);

        let path_vec = config.path_vec;
        assert!(path_vec.contains(&PathBuf::from(".")));
        assert!(path_vec.contains(&PathBuf::from("..")));
        assert!(path_vec.contains(&PathBuf::from("~")));
        
        assert!(!path_vec.contains(&PathBuf::from("-l")));
        assert!(!path_vec.contains(&PathBuf::from("-a")));
    }
}
