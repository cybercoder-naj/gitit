use std::{error::Error};
use std::sync::{Arc, Mutex};

use crate::global::state::State;

mod terminal;
mod global;
mod domain;

pub fn start() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(State::default()));
    State::listen(Arc::clone(&state));

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}