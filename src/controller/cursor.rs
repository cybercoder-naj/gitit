#[derive(PartialEq)]
pub enum Section {
    Files, Buttons
}

#[derive(PartialEq)]
pub enum Button {
    SelectAll
}

pub enum CursorAction {
    Up, Down, Left, Right, Select, Enter
}

pub struct Cursor {
    section: Section,
    button: Button,
    file_index: u8,
    num_files: Option<usize>
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            section: Section::Files,
            button: Button::SelectAll,
            file_index: 0,
            num_files: None
        }
    }

    pub fn is_in(&self, section: Section) -> bool {
        self.section == section
    }

    pub fn set_num_files(&mut self, num_files: usize) {
        self.num_files = Some(num_files);
    }

    pub fn try_dec_file_index(&mut self) -> Result<(), ()> {
        if self.file_index <= 0 {
            return Err(());
        }

        self.file_index -= 1;
        Ok(())
    }

    pub fn try_inc_file_index(&mut self) -> Result<(), ()> {
        if self.file_index as usize >= self.num_files.expect("Did not set num_files.") - 1 {
            return Err(());
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
}