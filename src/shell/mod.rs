use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub mod prompt;
pub mod commands;
pub mod git_branch;

pub fn run_shell() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();

    loop {
        let input = prompt::custom_prompt(&mut stdout);
        commands::handle_commands(input);
    }
}

