pub mod state;

use std::process::{Command, Stdio};
use regex::Regex;
use state::GitState;

pub fn init(state: &mut GitState) {
    let git_diff = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Git Diff didn't work");

    let cat = Command::new("cat")
        .stdin(Stdio::from(git_diff.stdout.unwrap()))
        .output()
        .expect("Cat piped didn't work");

    let delim = Regex::new(r"\s").expect("Invalid Regex");
    let output = String::from_utf8_lossy(&cat.stdout);

    let unstaged_files: Vec<_> = delim
        .split(output.as_ref())
        .filter(|file| !(*file).is_empty())
        .map(String::from)
        .collect();

    state.set_unstaged_files(unstaged_files);
}