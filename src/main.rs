use std::io::{self, Write};
mod command_list;
use crate::command_list::arts::welcome;


fn main() {
    
    welcome();

    let mut buf = String::new();

    loop {
        buf.clear();

        print!("\nλ ❯");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Buffer flush error: {}", e);
        }

        let bytes_read = io::stdin().read_line(&mut buf).unwrap_or(0);
        if bytes_read == 0 {
            println!("\nBye!\n");
            break;
        }

        if buf.trim().is_empty() { continue; }

        if let Some(cmd) = command_list::ShellCommand::parse(&buf) {
            cmd.run();
        }     
    }
}
