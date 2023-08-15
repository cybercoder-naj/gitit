use crate::{domain, global};

pub struct ModifiedFile {
    filename: String,
    staged: bool,
}

impl ModifiedFile {
    pub fn new(filename: String, staged: bool) -> Self {
        ModifiedFile {
            filename,
            staged,
        }
    }

    pub fn name(&self) -> String {
        self.filename.clone()
    }

    pub fn is_staged(&self) -> bool {
        self.staged
    }

    pub fn set_staged(&mut self) {
        match domain::stage_file(self) {
            true => self.staged = true,
            false => {
                panic!("something went horribly wrong");
            }
        }
    }

    pub fn unset_staged(&mut self) {
        match domain::restore_file(self) {
            true => self.staged = false,
            false => {
                panic!("something went horribly wrong");
            }
        }
    }

    pub fn toggle_staged(&mut self) {
        match self.staged {
            true => self.unset_staged(),
            false => self.set_staged()
        }
    }
}
