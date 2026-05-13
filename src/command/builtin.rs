use crate::arts;
use crate::quote;

pub fn run_exit() {
    println!("\nBye!");
    std::process::exit(0);
}

pub fn run_help() {
        println!(
        "exit  - close terminal app\n\
         ascii - displays a drawing\n\
         quote - displays a quote\n\
         help  - show this message"
    );
}

pub fn run_ascii() {
    arts::show_random_art();
}
 
pub fn run_quote() {
    quote::show_random_quote();
}
