use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::controller::{cursor::CursorAction, state::State};

pub fn listen(state: &mut State) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Up => state.do_cursor_action(CursorAction::Up),
                KeyCode::Down => state.do_cursor_action(CursorAction::Down),
                KeyCode::Char(' ') => state.do_cursor_action(CursorAction::Select),
                KeyCode::Enter => state.do_cursor_action(CursorAction::Enter),
                _ => {}
            };
        }
    }

    Ok(true)
}
