use std::error::Error;
use crate::controller::init;
use crate::controller::state::State;

pub mod terminal;
pub mod controller;

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut state = State::new();
    init(&mut state);

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, &mut state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}