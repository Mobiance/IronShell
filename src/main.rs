use std::{
    env, io:: Write, path::Path, process::{Command, Stdio}
};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use git2::Repository;

fn get_current_git_branch() -> Option<String> {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return None,
    };

    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return None,
    };

    let branch = match head.shorthand() {
        Some(branch) => branch,
        None => return None,
    };

    Some(branch.to_string())
}

fn custom_prompt(stdout: &mut StandardStream) -> String {

    let current_dir = match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            String::from("?")
        }
    };

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
    print!("$");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
    print!(" {}", current_dir);

    if let Some(branch) = get_current_git_branch() {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
        print!(" on ({}) ", branch);
        stdout.reset().unwrap();
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    print!(" >> ");

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string() 
}

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();

    println!("Type a command to run:");

    loop {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();

        let input = custom_prompt(&mut stdout);

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
                    } else {
                        let output = Command::new("eza")
                            .stdout(Stdio::piped())
                            .output()
                            .expect("Failed to execute `eza` command");

                        let stdout = String::from_utf8(output.stdout).expect("Not UTF8");
                        println!("files in {}:\n{}", root.display(), stdout);
                    }

                    previous_command = None;
                },
                "exit" => return,
                "mkdir" => {
                    let dir_path = args.peekable().peek().map_or("/", |x| *x);
                    match std::fs::create_dir_all(dir_path) {
                        Ok(_) => println!("Directory created: {}", dir_path),
                        Err(e) => eprintln!("Error creating directory: {}", e),
                    }
                },

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

