use super::ShellCommand;
use super::{builtin, cd, sys};

pub fn execute(command: ShellCommand) {
    match command {
        ShellCommand::Exit        => builtin::run_exit(),
        ShellCommand::Help        => builtin::run_help(),
        ShellCommand::Ascii       => builtin::run_ascii(),
        ShellCommand::Quote       => builtin::run_quote(),
        ShellCommand::Cd(path)    => cd::run_cd(path),
        ShellCommand::Sys(raw)    => sys::run_sys(raw),
    }
}
