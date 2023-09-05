use std::{error::Error, rc::Rc, cell::RefCell};
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
    let mut state = State::new();

    state.set_files(domain::retrieve_files_from_git());
    state.listen();

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}