use std::{error::Error, time::Duration, rc::Rc, cell::RefCell};

use crossterm::event::{self, Event, KeyCode};
use event_emitter_rs::EventEmitter;

use crate::global::cursor::CursorAction;

pub fn listen(event_emitter: Rc<RefCell<EventEmitter>>) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Char('k') => {
                    println!("HIII");
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::Up);
                }
                KeyCode::Char('j') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::Down);
                }
                KeyCode::Char('H') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::SuperLeft);
                }
                KeyCode::Char('L') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::SuperRight);
                }
                KeyCode::Char('h') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::Left);
                }
                KeyCode::Char('l') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::Right);
                }
                KeyCode::Char('J') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::SuperDown);
                }
                KeyCode::Char('K') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::SuperUp);
                }
                KeyCode::Char(' ') => {
                    event_emitter.borrow_mut().emit("cursor_action", CursorAction::Select);
                }
                _ => {}
            };
        }
    }

    Ok(true)
}
