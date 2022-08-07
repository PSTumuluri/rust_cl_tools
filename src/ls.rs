mod config;
mod long_list_utils;
mod file;

use std::error::Error;
use std::io;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::ls::config::Config;
use crate::ls::file::{File, Directory, LsError};

/// The ls command lists files in a directory.
pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {

    let config = Config::parse_args(&args);

    let mut error_vec: Vec<LsError> = vec![];
    let mut file_vec: Vec<File> = vec![]; 
    let mut dir_vec: Vec<Directory> = vec![];

    for path_name in config.path_name_vec.iter() {
        let path = Path::new(path_name);
        if path.is_file() {
            file_vec.push(File { 
                file_str: path_name,
            });
        } else if path.is_dir() {
            dir_vec.push(Directory {
                dir_str: path_name,
                file_name_vec: path.read_dir().unwrap()
                    .map(|dir_entry| 
                         dir_entry.unwrap().file_name().into_string().unwrap())
                    .collect(),
            });
        } else {
            error_vec.push(LsError {
                error_str: format!("could not find file or directory: {}", path_name),
            });
        }
       /*
        if let Err(_) = process_path(path, &config) {
                println!("Could not read directory: {}", path.display());
        }
        */
    }
    print_errors(&error_vec, &config);
    print_files(&file_vec, &config);
    print_dirs(&dir_vec, &config);
 

    Ok(())
}

fn print_errors(error_vec: &[LsError], config: &Config) {
    for error in error_vec {
        println!("{}", error.error_str);
    }
}

fn print_files(file_vec: &[File], config: &Config) {
    for file in file_vec {
        println!("{}", file.file_str);
    }
}

fn print_dirs(dir_vec: &[Directory], config: &Config) {
    for dir in dir_vec {
        println!("{}:", dir.dir_str);
        for file_name in dir.file_name_vec.iter() {
            println!("{}", file_name);
        }
    }
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
        println!("Could not list file: '{}'", path.display());
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
        let config = Config::parse_args(&args);

        assert!(config.list_all);

        let path_name_vec = config.path_name_vec;
        assert!(path_name_vec.contains(&"."));
        assert!(path_name_vec.contains(&".."));
        assert!(path_name_vec.contains(&"~"));
        
        assert!(!path_name_vec.contains(&"-l"));
        assert!(!path_name_vec.contains(&"-a"));
    }
}
