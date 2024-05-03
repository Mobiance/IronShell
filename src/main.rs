use std::{io::{stdout, Write}, process::Command};

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

    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .unwrap();

    child.wait().unwrap();
    }
}

