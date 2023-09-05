use std::{error::Error, rc::Rc, cell::RefCell};
use std::sync::{Arc, Mutex};
use event_emitter_rs::EventEmitter;
use crate::global::{event_emitter, state};
use crate::global::cursor::CursorAction;

use crate::global::state::State;

mod terminal;
mod global;
mod domain;

#[macro_use]
extern crate lazy_static;

pub fn start() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(State::new()));

    state.lock().unwrap().set_files(domain::retrieve_files_from_git());
    State::listen(Arc::clone(&state));

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}