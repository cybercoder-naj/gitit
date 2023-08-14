pub mod cursor;
pub mod models;
pub mod state;

use regex::Regex;
use state::State;
use std::process::Command;

use self::models::ModifiedFile;

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

pub fn get_diff_string(m_file: &ModifiedFile) -> String {
    let mut cmd = Command::new("git");

    cmd.arg("diff");
    if m_file.is_staged() {
        cmd.arg("--staged");
    }

    let output = cmd
        .arg(m_file.name().clone())
        .output()
        .expect("Git diff didn't work");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn stage_file(m_file: &ModifiedFile) -> bool {
    let add = Command::new("git")
        .arg("add")
        .arg(m_file.name())
        .output()
        .expect("git add didn't work");

    // check exit status of output
    true
}

pub fn restore_file(m_file: &ModifiedFile) -> bool {
    let restore = Command::new("git")
        .arg("restore")
        .arg("--staged")
        .arg(m_file.name())
        .output()
        .expect("git restore didn't work");

    // check exit status of output
    true
}
