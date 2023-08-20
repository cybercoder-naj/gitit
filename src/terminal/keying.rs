use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::global::{cursor::CursorAction, state::State};

pub fn listen(state: &mut State) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Char('k') => state.do_cursor_action(CursorAction::Up),
                KeyCode::Char('j') => state.do_cursor_action(CursorAction::Down),
                KeyCode::Char('H') => state.do_cursor_action(CursorAction::SuperLeft),
                KeyCode::Char('L') => state.do_cursor_action(CursorAction::SuperRight),
                KeyCode::Char('h') => state.do_cursor_action(CursorAction::Left),
                KeyCode::Char('l') => state.do_cursor_action(CursorAction::Right),
                KeyCode::Char('J') => state.do_cursor_action(CursorAction::SuperDown),
                KeyCode::Char('K') => state.do_cursor_action(CursorAction::SuperUp),
                KeyCode::Char(' ') => state.do_cursor_action(CursorAction::Select),
                _ => {}
            };
        }
    }

    Ok(true)
}
