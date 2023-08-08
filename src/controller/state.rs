pub(crate) struct ModifiedFile {
    pub(crate) filename: String,
    pub(crate) checked: bool
}

pub struct State {
    pub(crate) unstaged_files: Vec<ModifiedFile>,
    commit_msg: String,
    commit_desc: String,
    branch_name: String,
    pub(crate) current_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            unstaged_files: vec![],
            commit_msg: "".to_string(),
            commit_desc: "".to_string(),
            branch_name: "".to_string(),
            current_index: 0
        }
    }

    pub fn set_unstaged_files(&mut self, files: Vec<String>) {
        let files = files.iter().map(|name| ModifiedFile { filename: name.clone(), checked: false }).collect();
        self.unstaged_files = files;
    }
}