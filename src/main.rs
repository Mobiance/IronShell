use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    println!("Type a command to run:");
    loop {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
        print!("$ ");
        stdout.flush().unwrap();


        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next(){

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {

                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = std::env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => return,

                command => {

                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: std::process::Child| Stdio::from(output.stdout.unwrap()) 
                        );

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

                    match output  {
                        Ok(output) => { previous_command = Some(output);},
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
}

