pub mod parser;
pub mod executor;

#[path = "../command/builtin.rs"]
pub mod builtin;

#[path = "../command/cd.rs"]
pub mod cd;

#[path = "../command/sys.rs"]
pub mod sys;

pub enum ShellCommand<'a> {
    Help,
    Exit,
    Ascii,
    Quote,
    Cd(&'a str),
    Sys(&'a str),
}
