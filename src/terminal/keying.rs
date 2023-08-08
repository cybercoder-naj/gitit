use std::{
    error::Error,
    time::Duration
};

use crossterm::event::{
    self,
    Event,
    KeyCode
};

use crate::controller::state::State;
use crate::utils::{BTN_SECTION, BTN_SELECT_ALL, get_index_range};

pub fn listen(state: &mut State) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(250))? {
        let (min, max) = get_index_range(state);

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Up => {
                    if state.current_index > min {
                        state.current_index -= 1;
                    }
                },
                KeyCode::Down => {
                    if state.m_files.is_empty() {
                        return Ok(true);
                    }

                    if state.current_index < max {
                        state.current_index += 1;
                    }
                },
                KeyCode::Char(' ') => {
                    if state.current_index < 0 {
                        return Ok(true);
                    }

                    let m_file = &mut state.m_files[state.current_index as usize];
                    m_file.staged = !m_file.staged;
                }
                _ => {}
            };
        }

        if state.current_index == BTN_SECTION {
            state.button_index = BTN_SELECT_ALL;
        } else {
            state.button_index = -1;
        }
    }

    Ok(true)
}
