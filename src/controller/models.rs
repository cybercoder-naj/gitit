pub struct ModifiedFile {
    filename: String,
    staged: bool,
}

impl ModifiedFile {
    pub fn new(filename: String, staged: bool) -> Self {
        ModifiedFile {
            filename: filename,
            staged: staged,
        }
    }

    pub fn name(&self) -> String {
        self.filename.clone()
    }

    pub fn is_staged(&self) -> bool {
        self.staged
    }

    pub fn set_staged(&mut self) {
        self.staged = true;
    }

    pub fn unset_staged(&mut self) {
        self.staged = false;
    }

    pub fn toggle_staged(&mut self) {
        self.staged = !self.staged;
    }
}
