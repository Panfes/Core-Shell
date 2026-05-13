use std::io::{self, Write};
use crate::command::ShellCommand;

pub fn run() {
    let mut buf = String::new();

    loop {
        buf.clear();

        print!("\nλ ❯");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Buffer flush error: {}", e);
        }

        match io::stdin().read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => {
                eprintln!("Core-Shell: failed to read buf: {}", e);
                continue;
            }
        }

        if buf.trim().is_empty() { continue; }
        
        if let Some(cmd) = ShellCommand::parse(&buf) {
            cmd.run();
        }
    }

}

