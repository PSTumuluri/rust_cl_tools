use std::env;
use std::error::Error;
use std::io;
use std::fs::{self, DirEntry, ReadDir};

/// Represents the configuration for this command.
struct Config {
    dir_path_vec: Vec<String>,
    long_list: bool,
    list_all: bool,
}

impl Config {
    /// Constructs a default config object where all options are off.
    fn from_default() -> Config {
        Config {
            dir_path_vec: vec![],
            long_list: false,
            list_all: false,
        }
    }
}

/// The ls command lists files in a directory.
fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args)?;
    for dir_path in &config.dir_path_vec {
        if let Err(_) = process_dir_path(dir_path, &config) {
            println!("Directory not found: {}", dir_path);
        }
    }

    Ok(())
}

/// Scans command line arguments and returns a configuration option with the corresponding
/// options set and paths added.
/// Returns an error only if a specified option does not exist.
fn parse_config(args: &[String]) -> Result<Config, &str> {
    let mut config = Config::from_default();

    for arg in &args[1..] {
        let bytes = arg.as_bytes();
        // Options start with '-'. Anything else should be treated as a path name.
        if bytes[0] == b'-' {
            for byte in &bytes[1..] {
                match byte {
                    b'a'  => config.list_all = true,
                    b'l'  => config.long_list = true,
                    other => return Err("Option not recognized."),
                }
            }
        } else {
            config.dir_path_vec.push(String::from(arg));
        }
    }

    if config.dir_path_vec.is_empty() {
        config.dir_path_vec.push(String::from("."));
    }

    Ok(config)
}

/// Visits the specified path, printing its information if a file, or printing its 
/// contents' information if a directory.
/// Returns an error if the path does not correspond to an existing directory.
fn process_dir_path(dir_path: &String, config: &Config) -> io::Result<()> {
    let dir_iter = fs::read_dir(dir_path)?;
    for entry in dir_iter {
        let entry = entry?;
        process_entry(&entry, config);
    }

    Ok(())
}

/// Displays a directory entry according the configured settings.
/// Panics when the OsString cannot be converted to a UTF-8 string because I don't know
/// what else to do.
fn process_entry(entry: &DirEntry, config: &Config) {
    let entry_string = entry.file_name().into_string().unwrap();
    if config.list_all || (!config.list_all && entry_string.as_bytes()[0] != b'.') {
        println!("{}", entry_string);
    }
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

        let dir_path_vec = config.dir_path_vec;
        assert!(dir_path_vec.contains(&String::from(".")));
        assert!(dir_path_vec.contains(&String::from("..")));
        assert!(dir_path_vec.contains(&String::from("~")));
        
        assert!(!dir_path_vec.contains(&String::from("-l")));
        assert!(!dir_path_vec.contains(&String::from("-a")));
    }
}
