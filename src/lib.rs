use std::error::Error;

use crate::global::state::State;

mod terminal;
mod global;
mod domain;

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut state = State::default();
    init(&mut state);

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, &mut state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}

pub fn init(state: &mut State) {
    let raw_files = domain::retrieve_files_from_git();
    state.set_files(raw_files);
}
