use super::ShellCommand;

pub fn parse_line(input: &str) -> Option<ShellCommand<'_>> {
    let input = input.trim();

    let (cmd, args) = match input.split_once(' ') {
        Some((first, rest)) => (first, rest),
        None => (input, ""),
    };

    match cmd {
        "help"  => Some(ShellCommand::Help),
        "exit"  => Some(ShellCommand::Exit),
        "ascii" => Some(ShellCommand::Ascii),
        "quote" => Some(ShellCommand::Quote),
        "cd"    => Some(ShellCommand::Cd(args)),
        _       => Some(ShellCommand::Sys(input)),
    }
}
