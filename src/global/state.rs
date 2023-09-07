use std::sync::{Arc, Mutex};

use crate::global::{cursor::{Cursor, CursorAction, Section}, event_emitter};
use crate::global::constants::EVENT_SWITCH_SECTION;

#[derive(PartialEq)]
pub enum ButtonIndex {
    Commit,
    CommitAndPush,
    Quit,
}

impl Default for ButtonIndex {
    fn default() -> Self {
        ButtonIndex::Commit
    }
}

#[derive(Default)]
pub struct State {
    cursor: Cursor,
    button_index: ButtonIndex,
    current_section: Section
}

impl State {
    pub fn listen(state: Arc<Mutex<Self>>) {
        event_emitter().on(EVENT_SWITCH_SECTION, move |cursor_action: CursorAction| {
            let state = &mut *state.lock().unwrap();

            match cursor_action {
                CursorAction::Up => {
                    match state.current_section {
                        Section::Buttons => {
                            state.current_section = Section::Files
                        }
                        _ => ()
                    }
                }
                CursorAction::Down => {
                    match state.current_section {
                        Section::Files | Section::Diff => {
                            state.current_section = Section::Buttons
                        }
                        _ => ()
                    }
                }
                CursorAction::Left => {}
                CursorAction::Right => {}
                _ => ()
            }
        });
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

    // enigo::Enigo::new().key_click(Key::Layout('q'))
}

