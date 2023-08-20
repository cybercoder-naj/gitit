#[derive(PartialEq)]
pub enum Section {
    Files,
    FileControls,
    Diff,
    Buttons
}

pub enum CursorAction {
    Up,
    Down,
    Left,
    Right,
    SuperUp,
    SuperDown,
    SuperLeft,
    SuperRight,
    Select,
}

pub enum CursorError {
    OutOfBounds,
    NoFileExists,
}

pub struct Cursor {
    section: &'static Section,
    file_index: usize,
    num_files: Option<usize>,
    diff_scroll_offset: (u16, u16),
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            section: &Section::FileControls,
            file_index: 0,
            num_files: None,
            diff_scroll_offset: (0, 0),
        }
    }
}

impl Cursor {
    pub fn is_in(&self, section: &'static Section) -> bool {
        self.section == section
    }

    pub fn get_section(&self) -> &'static Section {
        self.section
    }

    pub fn set_num_files(&mut self, num_files: usize) {
        self.num_files = Some(num_files);
    }

    pub fn try_dec_file_index(&mut self) -> Result<(), CursorError> {
        let num_files = self.num_files.expect("Did not set num_files");
        if num_files == 0 {
            return Err(CursorError::NoFileExists);
        }

        if self.file_index <= 0 {
            return Err(CursorError::OutOfBounds);
        }

        self.file_index -= 1;
        Ok(())
    }

    pub fn try_inc_file_index(&mut self) -> Result<(), CursorError> {
        let num_files = self.num_files.expect("Did not set num_files");
        if num_files == 0 {
            return Err(CursorError::NoFileExists);
        }

        if self.file_index >= num_files - 1 {
            return Err(CursorError::OutOfBounds);
        }

        self.file_index += 1;
        Ok(())
    }

    pub fn get_file_index(&self) -> usize {
        self.file_index
    }

    pub fn move_to(&mut self, section: &'static Section) {
        self.section = section;
    }

    pub fn move_to_index(&mut self, section: &'static Section, index: usize) {
        self.move_to(section);
        match section {
            Section::Files => {
                self.file_index = index
            }
            Section::FileControls => {}
            Section::Diff => {}
            _ => {}
        }
    }

    pub fn reset_diff_scroll(&mut self) {
        self.diff_scroll_offset = (0, 0)
    }

    pub fn diff_scroll_up(&mut self) {
        if self.diff_scroll_offset.0 == 0 {
            return;
        }
        self.diff_scroll_offset = (self.diff_scroll_offset.0 - 1, self.diff_scroll_offset.1)
    }

    pub fn diff_scroll_down(&mut self) {
        self.diff_scroll_offset = (self.diff_scroll_offset.0 + 1, self.diff_scroll_offset.1)
    }

    pub fn get_diff_scroll(&self) -> (u16, u16) {
        self.diff_scroll_offset
    }
}
