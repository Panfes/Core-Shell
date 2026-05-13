use std::{env, process::Command};
use std::path::Path;
pub mod arts;
pub mod quote;


pub enum ShellCommand<'a> {
    Help,
    Exit,
    Ascii,
    Quote,
    Cd(&'a str),
    Sys(&'a str),
}

impl<'a> ShellCommand<'a> {
    pub fn parse(input: &'a str) -> Option<Self> {
        let input = input.trim();

        let (cmd, args) = match input.split_once(' ') {
            Some((first, rest)) => (first, rest),
            None => (input, ""),
        };

        match cmd {
            "help" => Some(ShellCommand::Help),
            "exit" => Some(ShellCommand::Exit),
            "ascii" => Some(ShellCommand::Ascii),
            "quote" => Some(ShellCommand::Quote),
            "cd" => Some(ShellCommand::Cd(args)),
            _ => Some(ShellCommand::Sys(input)), 
        }
    }

    pub fn run(&self) {
        match self {
            Self::Exit => {
                println!("Bye!\n");
                std::process::exit(0);
            },
            Self::Ascii => {
                arts::show_random_art();
            }
            Self::Help => {
                println!(
                    "exit - close terminal app\nascii - displays a drawing\nquote - displays a quote\nhelp - show this message"
                );
            }
            Self::Quote => quote::show_random_quote(),
            Self::Sys(raw) => {
                let mut parts = raw.split_whitespace();
                let program = match parts.next() {
                    Some(name) => name,
                    None => return,
                };

                let result = Command::new(program)
                    .args(parts)
                    .spawn();

                match result {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(e) => {
                        eprintln!("Core-Shell: ¯\\_(ツ)_/¯ '{}' not found ({})", program, e);
                    }
                }

            }
            Self::Cd(path_str) => {
                let path_str = path_str.trim();
                if path_str.is_empty() {
                    eprintln!("Core-Shell: cd: path required");
                    return;
                }

                let new_path = Path::new(path_str);
                if let Err(e) = env::set_current_dir(new_path) {
                    eprintln!("Core-Shell: cd: failed to chande directory to '{}': {}", path_str, e );
                }
            }

        }

    }
}

