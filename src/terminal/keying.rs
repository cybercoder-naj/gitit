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

pub fn listen(state: &mut State) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Up => {
                    if state.current_index > 0 {
                        state.current_index -= 1;
                    }
                },
                KeyCode::Down => {
                    if state.current_index < state.unstaged_files.len() - 1 {
                        state.current_index += 1;
                    }
                },
                KeyCode::Char(' ') => {
                    let m_file = &mut state.unstaged_files[state.current_index];
                    m_file.checked = !m_file.checked;
                }
                _ => {}
            };
        }
    }

    Ok(true)
}