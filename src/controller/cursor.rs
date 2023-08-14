#[derive(PartialEq)]
pub enum Section {
    Files,
    Buttons,
}

#[derive(PartialEq)]
pub enum Button {
    SelectAll,
}

pub enum CursorAction {
    Up,
    Down,
    Left,
    Right,
    Select,
    Enter,
}

pub enum CursorError {
    OutOfBounds,
    NoFileExists,
}

pub struct Cursor {
    section: Section,
    button: Button,
    file_index: u8,
    num_files: Option<usize>,
    diff_scroll_offset: (u16, u16),
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            section: Section::Files,
            button: Button::SelectAll,
            file_index: 0,
            num_files: None,
            diff_scroll_offset: (0, 0),
        }
    }

    pub fn is_in(&self, section: Section) -> bool {
        self.section == section
    }

    pub fn set_num_files(&mut self, num_files: usize) {
        self.num_files = Some(num_files);
    }

    pub fn try_dec_file_index(&mut self) -> Result<(), CursorError> {
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

        if self.file_index as usize >= num_files - 1 {
            return Err(CursorError::OutOfBounds);
        }

        self.file_index += 1;
        Ok(())
    }

    pub fn get_file_index(&self) -> u8 {
        self.file_index
    }

    pub fn get_button(&self) -> &Button {
        &self.button
    }

    pub fn move_to(&mut self, section: Section) {
        self.section = section;
    }

    pub fn reset_diff_scroll(&mut self) {
        self.diff_scroll_offset = (0, 0)
    }

    pub fn diff_scroll_up(&mut self) {
        self.diff_scroll_offset = (self.diff_scroll_offset.0 + 1, self.diff_scroll_offset.1)
    }

    pub fn diff_scroll_down(&mut self) {
        self.diff_scroll_offset = (self.diff_scroll_offset.0 - 1, self.diff_scroll_offset.1)
    }
    
    pub fn get_diff_scroll(&self) -> (u16, u16) {
        self.diff_scroll_offset
    }
}
