pub mod state;
pub mod cursor;

use std::process::Command;
use regex::Regex;
use state::State;

pub fn init(state: &mut State) {
    let raw_files = retrieve_files_from_git();
    state.set_files(raw_files);
}

fn retrieve_files_from_git() -> Vec<String> {
    let git_files = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .expect("Git Status didn't work");

    let delim = Regex::new(r"[\n\r]").expect("Invalid Regex");
    let output = String::from_utf8_lossy(&git_files.stdout);

    delim
        .split(output.as_ref())
        .filter(|file| !(*file).is_empty())
        .map(String::from)
        .collect()
}