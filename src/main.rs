mod ls;

use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Demoing the ls command:");
    ls::run(args)?;
    
    Ok(())
}
