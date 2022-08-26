use std::str::Chars;
use std::cmp::Ordering;
use std::fs::DirEntry;

/// The Settings struct represents the options enabled for this command.
/// Currently, the list all, long list, and reverse sort options are supported.
/// Each option is set upon reading a specific character, e.g. reading 'a' enables 
/// the list all option.
pub struct Settings {
    list_all: bool,
    long_list: bool,
    sort: Sort,
}

impl Settings {
    /// Settings prior to parsing any command line arguments.
    pub fn default() -> Settings {
        Settings {
            list_all: false,
            long_list: false,
            sort: Sort::default(),
        }
    }

    /// Attempts to apply the settings specified by the chars in the string.
    pub fn try_apply_settings(&mut self, chars: Chars) {
        for c in chars {
            match c {
                'a' => self.list_all = true,
                'l' => self.long_list = true,
                'r' => self.sort.reverse(),
                _ => eprintln!("error: option not recognized: {}", c),
            };
        }
    }

    pub fn sort(&self) -> &Sort {
        &self.sort
    }
}

pub enum SortType {
    Alphabetic,
    CreationTime,
}

pub struct Sort {
    sort_type: SortType,
    reversed: bool,
}

impl Sort {
    pub fn default() -> Sort {
        Sort {
            sort_type: SortType::Alphabetic,
            reversed: false,
        }
    }

    pub fn reverse(&mut self) {
        self.reversed = true;
    }

    pub fn reversed(&self) -> bool {
        self.reversed
    }
    
    pub fn sort_type(&self) -> &SortType {
        &self.sort_type
    }
}
