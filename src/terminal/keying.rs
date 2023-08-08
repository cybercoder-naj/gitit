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
                    // if state.index >  {
                    //     state.index -= 1;
                    // }
                },
                KeyCode::Down => {
                    if state.m_files.is_empty() {
                        return Ok(true);
                    }

                    // if state.index < max {
                    //     state.index += 1;
                    // }
                },
                KeyCode::Char(' ') => {
                    // if state.index < 0 {
                    //     return Ok(true);
                    // }

                    // let m_file = &mut state.m_files[state.index as usize];
                    // m_file.staged = !m_file.staged;
                }
                _ => {}
            };
        }
    }

    Ok(true)
}
