mod ls;

use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: cargo run <cmd> [options ...");
        return Ok(());
    }

    let cmd_args = &args[1..];
    if args[1].as_str() == "ls" {
        ls::run(cmd_args)?;
    } else {
        println!("Command not recognized: {}", args[1]);
    }

    Ok(())
}
