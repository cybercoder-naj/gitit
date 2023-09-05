use std::{error::Error};
use std::sync::{Arc, Mutex};




use crate::global::state::State;

mod terminal;
mod global;
mod domain;

pub fn start() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(State::new()));

    state.lock().unwrap().set_files(domain::retrieve_files_from_git());
    State::listen(Arc::clone(&state));

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}