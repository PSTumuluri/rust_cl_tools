use std::error::Error;
use std::fs::{self, DirEntry, ReadDir};

/// The ls command lists files in a directory.
fn main() -> Result<(), Box<dyn Error>> {

    fs::read_dir(".")?
        .for_each(|file| 
                  println!("{}", file.unwrap().file_name().into_string().unwrap()));

    Ok(())  
}
