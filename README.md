# rust-cl-tools

Unix-inspired command line tools written in Rust.

To demo a command, run

    $ cargo run <cmd> [options ...]

Currently, only the ls command is supported.

## ls

List directory entries. Supports the following options:

### list all (-a)

List hidden files, i.e. those whose file name begins with a dot.
