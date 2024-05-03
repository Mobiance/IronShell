use std::{io::{stdout, Write}, process::Command};

fn main() {
    loop {
    println!("Type a command to run:");
    print!("$ ");
    stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let command = input.trim();

    let mut child = Command::new(command)
        .spawn()
        .unwrap();

    child.wait().unwrap();
    }
}

