use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use std::env;
use crate::shell::git_branch;
use std::io::Write;

pub fn custom_prompt(stdout: &mut StandardStream) -> String {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            String::from("?")
        }
    };

    stdout
       .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
       .unwrap();
    print!("$");
    stdout
       .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
       .unwrap();
    print!(" {}", current_dir);

    if let Some(branch) = git_branch::get_current_git_branch() {
        stdout
           .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
           .unwrap();
        print!(" on  ({}) ", branch);
        stdout.reset().unwrap();
    }

    stdout
       .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
       .unwrap();
    print!(" >> ");

    stdout
       .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
       .unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

