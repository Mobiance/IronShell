use std::{
    io::{stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        println!("Type a command to run:");
        print!("$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = std::env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child  {
                    Ok(mut child) => { child.wait().unwrap(); },
                    Err(e) => { eprintln!("{}", e);
                }
                }
            }
        }
    }
}

