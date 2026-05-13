use std::env;
use std::path::Path;
 
pub fn run_cd(path_str: &str) {
    let path_str = path_str.trim();
 
    if path_str.is_empty() {
        eprintln!("Core-Shell: cd: path required");
        return;
    }
 
    if let Err(e) = env::set_current_dir(Path::new(path_str)) {
        eprintln!("Core-Shell: cd: failed to change directory to '{}': {}", path_str, e);
    }
}
