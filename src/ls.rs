use std::env;

use std::error::Error;
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

    Ok(())
}

fn parse_config(args: &[String]) -> Result<Config, String> {
    let mut config = Config::from_default();

    for arg in &args[1..] {
        let bytes = arg.as_bytes();
        // Options start with '-'. Anything else should be treated as a path name.
        if bytes[0] == b'-' {
            for byte in &bytes[1..] {
                match byte {
                    b'a'  => config.list_all = true,
                    b'l'  => config.long_list = true,
                    other => return Err(String::from("Option not recognized.")),
                }
            }
        } else {

        }
    }

    Ok(config)
}

fn display_entry(entry: &DirEntry, config: &Config) {
    println!("{}", entry.file_name().into_string().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_no_options_set() {
        let args = vec![String::from(".")];
        let config = parse_config(&args).unwrap();
        assert_eq!(config.long_list, false);
        assert_eq!(config.list_all, false);
    }
}
