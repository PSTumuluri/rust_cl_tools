# rust-cl-tools

Unix-inspired command line tools written in Rust.

To demo a command, run

    $ cargo run <cmd> [options ...]

Currently, only the ls command is supported.

## ls

The ls command supports the following options:

### long list (-l)

The long list option prints metadata about each file.

### list all (-a)

The lost all options causes the command to include hidden files.
