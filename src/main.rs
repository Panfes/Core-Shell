pub mod arts;
pub mod quote;
pub mod shell;
pub mod utils;
 
fn main() {
    arts::welcome();
    shell::run();
}
