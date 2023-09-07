use serde::{Serialize, Deserialize};

#[derive(PartialEq)]
pub enum Section {
    Files,
    Diff,
    Buttons
}

impl Default for Section {
    fn default() -> Self {
        Section::Files
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CursorAction {
    Up,
    Down,
    Left,
    Right,
    Select,
}

pub struct Cursor {
    section: &'static Section,
    diff_scroll_offset: (u16, u16),
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            section: &Section::Files,
            diff_scroll_offset: (0, 0),
        }
    }
}

impl Cursor {
    pub fn is_in(&self, section: &'static Section) -> bool {
        self.section == section
    }

    pub fn get_diff_scroll(&self) -> (u16, u16) {
        self.diff_scroll_offset
    }
}
