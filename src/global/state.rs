use enigo::{Key, KeyboardControllable};
use crate::global::{
    cursor::{Cursor, CursorAction, CursorError, Section},
    models::ModifiedFile,
};

pub enum FileControlState {
    None,
    Some,
    All
}

#[derive(PartialEq)]
pub enum ButtonIndex {
    Commit,
    CommitAndPush,
    Quit
}

impl Default for FileControlState {
    fn default() -> Self {
        FileControlState::None
    }
}

impl Default for ButtonIndex {
    fn default() -> Self {
        ButtonIndex::Commit
    }
}

#[derive(Default)]
pub struct State {
    m_files: Vec<ModifiedFile>,
    cursor: Cursor,
    file_control_state: FileControlState,
    button_index: ButtonIndex,
    popup_commit: bool
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

    pub fn get_button_index(&self) -> &ButtonIndex {
        &self.button_index
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
        let mut state = FileControlState::None;
        let mut c = 0usize;

        for m_file in &self.m_files {
            if m_file.is_staged() {
                state = FileControlState::Some;
                c += 1;
            }
        }
        if c == self.m_files.len() {
            state = FileControlState::All
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
                    },
                    Section::Buttons => {
                        // TODO make a beep sound
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
                    },
                    Section::Buttons => {
                        // TODO make a beep sound
                    }
                }
            }
            CursorAction::SuperLeft => {
                match self.cursor.get_section() {
                    Section::Files | Section::FileControls => {
                        // TODO make beep sound
                    }
                    Section::Diff | Section::Buttons => {
                        self.cursor.move_to(&Section::Files)
                    }
                }
            }
            CursorAction::SuperRight => {
                match self.cursor.get_section() {
                    Section::Files | Section::FileControls => {
                        self.cursor.move_to(&Section::Diff)
                    }
                    Section::Diff | Section::Buttons => {
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
                            FileControlState::None | FileControlState::Some => {
                                self.stage_all_files();
                            }
                            FileControlState::All => {
                                self.restore_all_files();
                            }
                        }
                    }
                    Section::Diff => {}
                    Section::Buttons => {
                        match self.button_index {
                            ButtonIndex::Quit => {
                                enigo::Enigo::new().key_click(Key::Layout('q'))
                            }
                            _ => {
                                self.popup_commit = true;
                            }
                        }
                    }
                }
            }
            CursorAction::SuperDown => {
                self.cursor.move_to(&Section::Buttons)
            }
            CursorAction::SuperUp => {
                self.cursor.move_to(&Section::Files)
            }
            CursorAction::Left => {
                if self.cursor.is_in(&Section::Buttons) {
                    match self.button_index {
                        ButtonIndex::CommitAndPush => {
                            self.button_index = ButtonIndex::Commit;
                        }
                        ButtonIndex::Quit => {
                            self.button_index = ButtonIndex::CommitAndPush;
                        }
                        ButtonIndex::Commit => {
                            // TODO make beep sound
                        }
                    }
                }
            }
            CursorAction::Right => {
                if self.cursor.is_in(&Section::Buttons) {
                    match self.button_index {
                        ButtonIndex::Commit => {
                            self.button_index = ButtonIndex::CommitAndPush;
                        }
                        ButtonIndex::CommitAndPush => {
                            self.button_index = ButtonIndex::Quit;
                        }
                        ButtonIndex::Quit => {
                            // TODO make beep sound
                        }
                    }
                }
            }
        }
    }
}
