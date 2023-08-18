use crate::global::{
    cursor::{Cursor, CursorAction, CursorError, Section},
    models::ModifiedFile,
};

pub enum FileControlState {
    NONE,
    SOME,
    ALL
}

impl Default for FileControlState {
    fn default() -> Self {
        FileControlState::NONE
    }
}

#[derive(Default)]
pub struct State {
    m_files: Vec<ModifiedFile>,
    cursor: Cursor,
    file_control_state: FileControlState
}

impl State {
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

    fn stage_all_files(&mut self) {
        for m_file in &mut self.m_files {
            m_file.set_staged()
        }
    }

    fn restore_all_files(&mut self) {
        for m_file in &mut self.m_files {
            m_file.unset_staged()
        }
    }

    pub fn configure_file_control_state(&mut self) {
        let mut state = FileControlState::NONE;
        let mut c = 0usize;

        for m_file in &self.m_files {
            if m_file.is_staged() {
                state = FileControlState::SOME;
                c += 1;
            }
        }
        if c == self.m_files.len() {
            state = FileControlState::ALL
        }

        self.file_control_state = state
    }

    pub fn get_file_control_state(&self) -> &FileControlState {
        &self.file_control_state
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
                    Section::Files | Section::FileControls => {
                        // TODO make beep sound
                    }
                    Section::Diff => {
                        self.cursor.move_to(&Section::Files)
                    }
                }
            }
            CursorAction::SuperRight => {
                match self.cursor.get_section() {
                    Section::Files | Section::FileControls => {
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
                        match self.file_control_state {
                            FileControlState::NONE | FileControlState::SOME => {
                                self.stage_all_files();
                            }
                            FileControlState::ALL => {
                                self.restore_all_files();
                            }
                        }
                    }
                    Section::Diff => {}
                }
            }
        }
    }
}
