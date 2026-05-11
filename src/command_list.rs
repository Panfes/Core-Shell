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

        let cmd = input.split_whitespace().next()?;

        match cmd {
            "help" => Some(ShellCommand::Help),
            "exit" => Some(ShellCommand::Exit),
            "ascii" => Some(ShellCommand::Ascii),
            "quote" => Some(ShellCommand::Quote),
            _ => Some(ShellCommand::Sys(input.to_string())),
        }
    }

    pub fn run(&self) {
        match self {
            Self::Exit => std::process::exit(0),

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

                if let Some(program) = parts.next() {
                    let child = Command::new(program)
                        .args(parts)
                        .spawn();

                    match child {
                        Ok(mut process) => { let _ = process.wait(); },
                        Err(e) => { eprintln!("Core-Shell Command not found: {}", e); },
                    }
                }

            }

        }

    }
}

