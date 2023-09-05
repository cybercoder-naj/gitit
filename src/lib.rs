use std::{error::Error, rc::Rc, cell::RefCell};
use event_emitter_rs::EventEmitter;

use crate::global::state::State;

mod terminal;
mod global;
mod domain;

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut event_emitter = Rc::new(RefCell::new(EventEmitter::new()));
    let mut state = State::new(Rc::clone(&event_emitter));
    init(&mut state);

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, state, event_emitter)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}

pub fn init(state: &mut State) {
    let raw_files = domain::retrieve_files_from_git();
    state.set_files(raw_files);
}
