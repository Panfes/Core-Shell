use std::io::{self, Write};
mod command_list;
use crate::command_list::arts::welcome;


fn main() {
    
    welcome();

    let mut buf = String::new();

    loop {
        buf.clear();

        print!("\n λ ❯");
        match io::stdout().flush() {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
                panic!("aaa!!!");
            }
        }

        match io::stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!();
            },
        }

        if buf.trim().is_empty() { continue; }

        if let Some(cmd) = command_list::ShellCommand::parse(&buf) {
            cmd.run();
        } else {
            println!("Unknown command\ntype help for commands list");
        }
    }
}
