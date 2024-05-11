use std::env;
use std::path::Path;
use std::process::{Command, Stdio, exit};

pub fn handle_commands(input: String) {
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                } else {
                    if cfg!(target_os = "windows") {
                        let output = Command::new("cmd")
                            .arg("/C")
                            .arg("dir")
                            .stdout(Stdio::piped())
                            .output()
                            .expect("Failed to execute `dir` command");

                        let stdout = String::from_utf8(output.stdout).expect("Not UTF8");
                        println!("files in {}:\n{}", root.display(), stdout);
                    } else {
                        let output = Command::new("ls")
                            .stdout(Stdio::piped())
                            .output()
                            .expect("Failed to execute `ls` command");

                        let stdout = String::from_utf8(output.stdout).expect("Not UTF8");
                        println!("files in {}:\n{}", root.display(), stdout);
                    }
                }

                previous_command = None;
            }
            "exit" => exit(0),
            "mkdir" => {
                let dir_path = args.peekable().peek().map_or("/", |x| *x);
                match std::fs::create_dir_all(dir_path) {
                    Ok(_) => println!("Directory created: {}", dir_path),
                    Err(e) => eprintln!("Error creating directory: {}", e),
                }
            }
            "del" => {
                let target_path = args.peekable().peek().map_or("/", |x| *x);
                let path = Path::new(target_path);
                if path.is_dir() {
                    match std::fs::remove_dir_all(path) {
                        Ok(_) => println!("Directory deleted: {}", target_path),
                        Err(e) => eprintln!("Error deleting directory: {}", e),
                    }
                } else {
                    match std::fs::remove_file(path) {
                        Ok(_) => println!("File deleted: {}", target_path),
                        Err(e) => eprintln!("Error deleting file: {}", e),
                    }
                }
            }
            "ls" => {
                let dir_path = args.peekable().peek().map_or(".", |x| *x);
                match std::fs::read_dir(dir_path) {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                println!("{}", entry.file_name().to_string_lossy());
                            }
                        }
                    }
                    Err(e) => eprintln!("Error listing directory: {}", e),
                }
            }
            "powershell" => {
                let script_path = args.peekable().peek().map_or("", |x| x);
                let output = Command::new("powershell")
                    .arg("-File")
                    .arg(script_path)
                    .spawn()
                    .expect("Failed to execute PowerShell command");

                previous_command = Some(output);
            }
            "bash" => {
                let script_path = args.peekable().peek().map_or("", |x| x);
                let output = Command::new("bash")
                    .arg("-f")
                    .arg(script_path)
                    .spawn()
                    .expect("Failed to execute Bash command");

                previous_command = Some(output);
            }
            command => {
                let stdin = previous_command
                    .map_or(Stdio::inherit(), |output: std::process::Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                let stdout = if commands.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };
                let output = Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match output {
                    Ok(output) => {
                        previous_command = Some(output);
                    }
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }
    if let Some(mut final_command) = previous_command {
        final_command.wait().unwrap();
    }
}
