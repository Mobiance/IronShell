# Simple Shell in Rust

This is a simple shell implementation in Rust, allowing users to execute commands and pipe their output.

## Getting Started

To run the shell, make sure you have Rust installed on your system. You can install Rust by following the instructions on the official Rust website: https://www.rust-lang.org/.

Once Rust is installed, you can compile and run the shell using the following commands:

```bash
$ cargo run
```


## Features

    Command Execution: Users can type commands to run within the shell.
    Piping: Commands can be piped together using the | operator.
    Change Directory (cd): Users can change the current directory using the cd command.

## Usage

    When prompted, type a command you want to execute.
    Press Enter to run the command.
    To pipe commands, separate them with |. For example: ls -l | grep .rs.
    Use cd <directory> to change the current directory.

## Example

```bash
Type a command to run:
$ ls -l | grep .rs
-rwxr-xr-x 1 user user 12345 May 1 12:34 main.rs
```


## Notes

    This shell is a basic implementation and may not support all features of a standard shell.
    Error handling is minimal and may need improvement for robustness.
    Use caution when executing commands, especially those involving system operations.
