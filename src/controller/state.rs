pub(crate) struct ModifiedFile {
    pub(crate) filename: String,
    pub(crate) staged: bool,
}

pub struct State {
    pub(crate) m_files: Vec<ModifiedFile>,
    _commit_msg: String,
    _commit_desc: String,
    _branch_name: String,
    pub(crate) current_index: isize,
    pub(crate) button_index: isize
}

impl State {
    pub fn new() -> Self {
        Self {
            m_files: vec![],
            _commit_msg: String::new(),
            _commit_desc: String::new(),
            _branch_name: String::new(),
            current_index: 0,
            button_index: -1
        }
    }

    pub fn set_unstaged_files(&mut self, files: Vec<String>) {
        self.m_files = files
            .iter()
            .map(|name| {
                let flags = &name[..1];

                ModifiedFile {
                    filename: String::from(&name[3..]),
                    staged: !(flags == "??" || flags.starts_with(" "))
                }
            })
            .collect();
    }
}
