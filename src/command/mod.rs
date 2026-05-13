pub mod builtin;
pub mod cd;
pub mod sys;
 
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
            "help"  => Some(ShellCommand::Help),
            "exit"  => Some(ShellCommand::Exit),
            "ascii" => Some(ShellCommand::Ascii),
            "quote" => Some(ShellCommand::Quote),
            "cd"    => Some(ShellCommand::Cd(args)),
            _       => Some(ShellCommand::Sys(input)),
        }
    }
 
    pub fn run(&self) {
        match self {
            Self::Exit        => builtin::run_exit(),
            Self::Help        => builtin::run_help(),
            Self::Ascii       => builtin::run_ascii(),
            Self::Quote       => builtin::run_quote(),
            Self::Cd(path)    => cd::run_cd(path),
            Self::Sys(raw)    => sys::run_sys(raw),
        }
    }
}
