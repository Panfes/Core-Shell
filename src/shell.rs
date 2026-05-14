use std::io::{self, Write};
use crate::utils::{parser, executor};

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
                eprintln!("Core-Shell: failed to read Buffer: {}", e);
                continue;
            }
        }

        if buf.trim().is_empty() { continue; }
        
        if let Some(cmd) = parser::parse_line(&buf) {
            executor::execute(cmd);
        }
    }

}

