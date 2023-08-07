pub struct GitState {
    unstaged_files: Vec<String>,
    commit_msg: String,
    commit_desc: String,
    branch_name: String,
}

impl GitState {
    pub fn new() -> Self {
        Self {
            unstaged_files: vec![],
            commit_msg: "".to_string(),
            commit_desc: "".to_string(),
            branch_name: "".to_string(),
        }
    }

    pub fn set_unstaged_files(&mut self, files: Vec<String>) {
        self.unstaged_files = files;
    }
}