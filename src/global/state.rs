use crate::global::cursor::{Cursor, CursorAction, CursorError, Section};
use super::models::ModifiedFile;

pub struct State {
    m_files: Vec<ModifiedFile>,
    cursor: Cursor,
}

impl State {
    pub fn new() -> Self {
        Self {
            m_files: vec![],
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

        self.cursor_mut().set_num_files(files.len());
    }

    pub fn get_files(&self) -> &Vec<ModifiedFile> {
        &self.m_files
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn get_current_file(&self) -> Option<&ModifiedFile> {
        if self.m_files.len() == 0 {
            return None;
        }
        Some(&self.m_files[self.cursor.get_file_index()])
    }

    pub fn do_cursor_action(&mut self, action: CursorAction) {
        match action {
            CursorAction::Up => {
                match self.cursor.get_section() {
                    Section::Files => {
                        self.cursor.try_dec_file_index().unwrap_or_else(|e| match e {
                            CursorError::OutOfBounds => {
                                self.cursor.move_to(&Section::FileControls)
                            }
                            CursorError::NoFileExists => {
                                // TODO make a beep sound
                            }
                        });
                        self.cursor.reset_diff_scroll();
                    }
                    Section::FileControls => {
                        // TODO make a beep sound
                    }
                    Section::Diff => {
                        self.cursor.diff_scroll_up()
                    }
                }
            }
            CursorAction::Down => {
                match self.cursor.get_section() {
                    Section::Files => {
                        self.cursor.try_inc_file_index().unwrap_or_else(|e| match e {
                            CursorError::OutOfBounds | CursorError::NoFileExists => {
                                // TODO move to bottom window
                            }
                        });
                        self.cursor.reset_diff_scroll();
                    }
                    Section::FileControls => {
                        self.cursor.move_to_index(&Section::Files, 0);
                    }
                    Section::Diff => {
                        self.cursor.diff_scroll_down();
                    }
                }
            }
            CursorAction::SuperLeft => {
                match self.cursor.get_section() {
                    Section::Files | Section::FileControls=> {
                        // TODO make beep sound
                    }
                    Section::Diff => {
                        self.cursor.move_to(&Section::Files)
                    }
                }
            }
            CursorAction::SuperRight => {
                match self.cursor.get_section() {
                    Section::Files | Section::FileControls=> {
                        self.cursor.move_to(&Section::Diff)
                    }
                    Section::Diff => {
                        // TODO make beep sound
                    }
                }
            }
            CursorAction::Select => {
                match self.cursor.get_section() {
                    Section::Files => {
                        let m_file = &mut self.m_files[self.cursor.get_file_index()];
                        m_file.toggle_staged();
                    }
                    Section::FileControls => {

                    }
                    Section::Diff => {}
                }
            }
        }
    }
}
