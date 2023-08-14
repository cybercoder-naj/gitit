use super::cursor::{Button, Cursor, CursorAction, CursorError, Section};
use super::models::ModifiedFile;

pub struct State {
    pub m_files: Vec<ModifiedFile>,
    _commit_msg: String,
    _commit_desc: String,
    _branch_name: String,
    pub cursor: Cursor,
}

impl State {
    pub fn new() -> Self {
        Self {
            m_files: vec![],
            _commit_msg: String::new(),
            _commit_desc: String::new(),
            _branch_name: String::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn set_files(&mut self, files: Vec<String>) {
        self.m_files = files
            .iter()
            .map(|name| {
                let flags = &name[..1];

                ModifiedFile::new(
                    String::from(&name[3..]),
                    !(flags == "??" || flags.starts_with(" ")),
                )
            })
            .collect();

        self.cursor.set_num_files(self.m_files.len());
    }

    pub fn get_current_file(&self) -> &ModifiedFile {
        &self.m_files[self.cursor.get_file_index() as usize]
    }

    pub fn do_cursor_action(&mut self, action: CursorAction) {
        match action {
            CursorAction::Up => {
                if self.cursor.is_in(Section::Files) {
                    self.cursor.try_dec_file_index().unwrap_or_else(|_| {
                        // TODO make beep sound
                    });
                    self.cursor.reset_diff_scroll()
                } else if self.cursor.is_in(Section::Buttons) {
                    self.cursor.move_to(Section::Files);
                    self.cursor.reset_diff_scroll()
                } else if self.cursor.is_in(Section::Diff) {
                    self.cursor.diff_scroll_up();
                }
            }
            CursorAction::Down => {
                if self.cursor.is_in(Section::Files) {
                    self.cursor
                        .try_inc_file_index()
                        .unwrap_or_else(|e| match e {
                            CursorError::OutOfBounds => {
                                self.cursor.move_to(Section::Buttons);
                            }
                            CursorError::NoFileExists => {}
                        });
                    self.cursor.reset_diff_scroll()
                } else if self.cursor.is_in(Section::Buttons) {
                    // TODO make beep sound
                    self.cursor.reset_diff_scroll()
                } else if self.cursor.is_in(Section::Diff) {
                    self.cursor.diff_scroll_down();
                }
            }
            CursorAction::Left => {
                if self.cursor.is_in(Section::Diff) {
                    self.cursor.move_to(Section::Files)
                } else {
                    // TODO make beep sound
                }
            }
            CursorAction::Right => {
                if self.cursor.is_in(Section::Files) || self.cursor.is_in(Section::Buttons) {
                    self.cursor.move_to(Section::Diff)
                } else {
                    // TODO make beep sound
                }
            }
            CursorAction::Select => {
                if self.cursor.is_in(Section::Files) {
                    let m_file = &mut self.m_files[self.cursor.get_file_index() as usize];
                    m_file.toggle_staged();
                } else {
                    // TODO make a beep sound
                }
            }
            CursorAction::Enter => {
                if self.cursor.is_in(Section::Buttons) {
                    match self.cursor.get_button() {
                        Button::SelectAll => {
                            for m_file in &mut self.m_files {
                                m_file.set_staged();
                            }
                        }
                    }
                } else {
                    // TODO make a beep sound
                }
            }
        }
    }
}
