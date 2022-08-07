use std::path::PathBuf;

/// Represents the configuration for this command.
/// We can use string slices because the config lives and dies with the command line
/// arguments in the main function.
pub struct Config<'a> {
    pub path_name_vec: Vec<&'a str>,
    pub long_list: bool,
    pub list_all: bool,
}

impl<'a> Config<'a> {
    /// Constructs a default config object where all options are off.
    fn from_default() -> Config<'a> {
        Config {
            path_name_vec: vec![],
            long_list: false,
            list_all: false,
        }
    }

    /// Scans command line arguments and returns a configuration option with the 
    /// corresponding options set and paths added. 
    /// Returns an error only if a specified option does not exist.
    pub fn parse_args(args: &'a [String]) -> Config<'a> {
        let mut config = Config::from_default();

        for arg in &args[1..] {
            let bytes = arg.as_bytes();
            // Options consist of a '-' followed by at least one character.
            // Anything else should be treated as a path name.
            if bytes.len() > 1 && bytes[0] == b'-' {
                for &byte in &bytes[1..] {
                    config.set_option(byte);
                }
            } else {
                config.add_path_name(arg);
            }
        }

        if config.path_name_vec.is_empty() {
            config.add_path_name(".");
        }

        config
    }


    pub fn set_option(&mut self, byte: u8) {
        match byte {
            b'a'  => self.list_all = true,
            _ => println!("option not recognized"),
        };
    }

    pub fn add_path_name(&mut self, path: &'a str) {
        self.path_name_vec.push(path);
    }
}


