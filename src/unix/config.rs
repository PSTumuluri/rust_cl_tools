mod config_utils;

use std::path::PathBuf;
use std::error::Error;
use std::io::Error as IoError;
use std::io::Result as IoResult;
use std::io::{ErrorKind};
use std::fs::{DirEntry, ReadDir};
use std::cmp::Ordering;

use config_utils::{Settings, Sort, SortType};

/// The Config struct separates command line arguments into categories upon parsing.
/// The only categories right now are path names and settings. Any option that starts
/// with a '-' and has more than one character is treated as a potential setting. 
/// Everything else is treated as a potential path name.
/// We can use string slices to refer to the path names because the config lives and 
/// dies with the command line  arguments in the main function.
pub struct Config {
    regular_files: Vec<PathBuf>,
    directories: Vec<PathBuf>,
    settings: Settings,
}

impl Config {
    /// Configuration prior to parsing any command line arguments.
    fn default() -> Config {
        Config {
            regular_files: vec![],
            directories: vec![],
            settings: Settings::default(),
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Scans command line arguments and returns a configuration option with the 
    /// corresponding path names added and settings set.
    pub fn from_args(args: impl Iterator<Item = String>) -> Config {
        let mut config = Config::default();

        for arg in args {
            let mut chars = arg.chars();

            if let Some('-') = chars.next() {
                config.settings.try_apply_settings(chars);
            } else {
                let path = PathBuf::from(&arg);

                if path.is_file() {
                    config.regular_files.push(path);
                } else if path.is_dir() {
                    config.directories.push(path);
                }
                else {
                    eprintln!("error: could not find file or directory: {}", arg);
                }
            }
        }
        
        if config.regular_files.is_empty() && config.directories.is_empty() {
            config.directories.push(PathBuf::from("."));
        }

        config
    }

    pub fn print_regular_files(&self) {
        for file in self.regular_files.iter() {
            self.print_entry(file);
        }
    }

    pub fn print_directories(&self) -> Result<(), Box<dyn Error>> {
        for dir in self.directories.iter() {
            println!("{}:", dir.display());

            let read_dir = dir.read_dir()?;
            self.print_dir_entries(read_dir)?; 
        }

        Ok(())
    }

    fn print_dir_entries(&self, read_dir: ReadDir) -> Result<(), Box<dyn Error>> {
        let entries = self.get_sorted_dir_entries(read_dir)?;
        for entry in entries.iter() {
            let file_name = entry.file_name().into_string();
            match file_name {
                Err(_) => return Err(Box::new(IoError::new(ErrorKind::Other, "idk"))),
                Ok(name) => self.print_entry(&entry.path())?,
            };
        }

        Ok(())
    }

    fn print_entry(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        println!("{}", path.file_name().unwrap().to_str().unwrap());

        Ok(())
    } 
 
    fn get_sorted_dir_entries(&self, dir: ReadDir) -> IoResult<Vec<DirEntry>>
    {
        let mut entries = vec![];
        for result in dir {
            let entry = result?;
            entries.push(entry);
        }

        self.sort_dir_entries(&mut entries);
        Ok(entries)
    }

    fn sort_dir_entries(&self, entries: &mut [DirEntry]) {
        let sort = self.settings.sort();
        match sort.sort_type() {
            SortType::Alphabetic => {
                entries.sort_by(|lhs, rhs| {
                    let comp = lhs.file_name().cmp((&rhs.file_name()));
                    if sort.reversed() {
                        match comp {
                            Ordering::Less => Ordering::Greater,
                            Ordering::Greater => Ordering::Less,
                            Ordering::Equal => Ordering::Equal,
                        }
                    } else {
                        comp
                    }
                });
            },
            SortType::CreationTime => {
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_adds_directory_paths_and_sets_options() {
        let args: Vec<String> = vec![String::from("ls"), String::from("."), 
            String::from("-l"), String::from(".."), String::from("-a"), 
            String::from("/")];

        let config = Config::from_args(args.into_iter());

        assert!(config.settings.list_all);

        assert!(config.directories.contains(&PathBuf::from(".")));
        assert!(config.directories.contains(&PathBuf::from("..")));
        assert!(config.directories.contains(&PathBuf::from("/")));
        
        assert!(!config.directories.contains(&PathBuf::from("-l")));
        assert!(!config.directories.contains(&PathBuf::from("-a")));
        assert!(!config.regular_files.contains(&PathBuf::from("-l")));
        assert!(!config.regular_files.contains(&PathBuf::from("-a")));
    }
}
