use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use crate::global::constants::{EVENT_KEYPRESS, EVENT_SWITCH_SECTION};


use crate::global::cursor::CursorAction;
use crate::global::event_emitter;

pub fn listen() -> Result<bool, Box<dyn Error>> {
    let mut event_emitter = event_emitter();

    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Char('k') => {
                    event_emitter.emit(EVENT_KEYPRESS, CursorAction::Up);
                }
                KeyCode::Char('j') => {
                    event_emitter.emit(EVENT_KEYPRESS, CursorAction::Down);
                }
                KeyCode::Char('h') => {
                    event_emitter.emit(EVENT_KEYPRESS, CursorAction::Left);
                }
                KeyCode::Char('l') => {
                    event_emitter.emit(EVENT_KEYPRESS, CursorAction::Right);
                }
                KeyCode::Char('H') => {
                    event_emitter.emit(EVENT_SWITCH_SECTION, CursorAction::Left);
                }
                KeyCode::Char('L') => {
                    event_emitter.emit(EVENT_SWITCH_SECTION, CursorAction::Right);
                }
                KeyCode::Char('J') => {
                    event_emitter.emit(EVENT_SWITCH_SECTION, CursorAction::Down);
                }
                KeyCode::Char('K') => {
                    event_emitter.emit(EVENT_SWITCH_SECTION, CursorAction::Up);
                }
                KeyCode::Char(' ') => {
                    event_emitter.emit(EVENT_KEYPRESS, CursorAction::Select);
                }
                _ => {}
            };
        }
    }

    Ok(true)
}
