//format the readme in markdown
# IronShell

IronShell is a customizable and interactive command-line interface (CLI) tool written in Rust. It provides a user-friendly environment for executing commands, navigating directories, and interacting with Git repositories.
Features

- Customizable Prompt: IronShell offers a customizable prompt that displays the current directory and Git branch (if available).
- Built-in Commands: Includes built-in commands such as cd, exit, mkdir, del, ls, etc., for common file system operations.
- Pipeline Support: Supports command chaining using the | operator for creating pipelines.
- Git Integration: Displays the current Git branch in the prompt, allowing users to easily work with Git repositories.

## Installation

To install IronShell, ensure you have Rust installed on your system. You can install Rust by following the instructions on rustup.rs.
Once Rust is installed, you can install IronShell using Cargo, the Rust package manager:

```bash
cargo install IronShell
```

## Usage

After installing IronShell, you can start the shell by running:

```bash
IronShell
```

Once the shell is launched, you can type commands directly into the prompt. Here are some examples of common commands:

- Navigate to a directory:

```bash
cd path/to/directory
```
 - List files in the current directory:

```bash
ls
```

- Create a new directory:

```bash
mkdir new_directory
```

- Run a custom command:

```bash
custom_command arg1 arg2
```

## Contributing

Contributions to IronShell are welcome! If you encounter any issues, have feature requests, or would like to contribute code, please feel free to open an issue or submit a pull request on GitHub.

## License

IronShell is licensed under the MIT License. See the LICENSE file for details.
