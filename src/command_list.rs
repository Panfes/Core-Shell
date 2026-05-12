use std::process::Command;
pub mod arts;
pub mod quote;


pub enum ShellCommand {
    Help,
    Exit,
    Ascii,
    Quote,
    Sys(String),
}

impl ShellCommand {
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();

        let parts = input.split_once(' ');

        let cmd = match parts {
            Some((first, _)) => first,
            None => input,
        };

        match cmd {
            "help" => Some(ShellCommand::Help),
            "exit" => Some(ShellCommand::Exit),
            "ascii" => Some(ShellCommand::Ascii),
            "quote" => Some(ShellCommand::Quote),
            _ => Some(ShellCommand::Sys(input.to_string())), // Алокация без которой никуда
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

        }

    }
}

