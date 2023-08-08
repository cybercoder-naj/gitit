pub mod state;

use std::process::Command;
use regex::Regex;
use state::State;

pub fn init(state: &mut State) {
    let git_files = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .expect("Git Diff didn't work");

    let delim = Regex::new(r"[\n\r]").expect("Invalid Regex");
    let output = String::from_utf8_lossy(&git_files.stdout);

    let raw_files: Vec<_> = delim
        .split(output.as_ref())
        .filter(|file| !(*file).is_empty())
        .map(String::from)
        .collect();

    state.set_unstaged_files(raw_files);
}