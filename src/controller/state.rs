pub(crate) struct ModifiedFile {
    pub(crate) filename: String,
    pub(crate) checked: bool
}

pub struct State {
    pub(crate) unstaged_files: Vec<ModifiedFile>,
    _commit_msg: String,
    _commit_desc: String,
    _branch_name: String,
    pub(crate) current_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            unstaged_files: vec![],
            _commit_msg: String::new(),
            _commit_desc: String::new(),
            _branch_name: String::new(),
            current_index: 0
        }
    }

    pub fn set_unstaged_files(&mut self, files: Vec<String>) {
        self.unstaged_files = files
            .iter()
            .map(|name| ModifiedFile {
                filename: name.clone(),
                checked: false }
            )
            .collect();
    }
}