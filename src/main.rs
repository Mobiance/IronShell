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
            }
            command => {
                let mut child = Command::new(command).args(args).spawn().unwrap();

                child.wait().unwrap();
            }
        }
    }
}

