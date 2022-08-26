use std::error::Error;
use std::env;
use std::process;

/// The ls command lists directory entries.
fn main() -> Result<(), Box<dyn Error>> {

    let mut args = env::args();
    args.next(); // Burn program name
    if cfg!(unix) {
        rust_cl_tools::run_for_unix(args).unwrap_or_else(|err| {
            println!("Problem processing command line arguments: {:?}", err);
            process::exit(1);
        });
    } else {
        eprintln!("ls command not supported on operating system {}", env::consts::OS);
        process::exit(1);
    }

    Ok(())
}
