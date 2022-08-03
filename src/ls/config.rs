use std::path::PathBuf;

/// Represents the configuration for this command.
pub struct Config {
    pub path_vec: Vec<PathBuf>,
    pub long_list: bool,
    pub list_all: bool,
}

impl Config {
    /// Constructs a default config object where all options are off.
    pub fn from_default() -> Config {
        Config {
            path_vec: vec![],
            long_list: false,
            list_all: false,
        }
    }

    pub fn set_option(&mut self, byte: u8) -> Result<(), &'static str> {
        match byte {
            b'a'  => self.list_all = true,
            b'l'  => self.long_list = true,
            other => return Err("Option not recognized."),
        }

        Ok(())
    }

    pub fn add_path(&mut self, path: &str) {
        self.path_vec.push(PathBuf::from(path));
    }
}


