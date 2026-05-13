use std::process::Command;
 
pub fn run_sys(raw: &str) {
    let mut parts = raw.split_whitespace();
 
    let program = match parts.next() {
        Some(name) => name,
        None => return,
    };
 
    match Command::new(program).args(parts).spawn() {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("Core-Shell: ¯\\_(ツ)_/¯ '{}' not found ({})", program, e);
        }
    }
}

