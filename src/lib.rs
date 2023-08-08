use std::error::Error;
use crate::controller::state::State;

mod terminal;
pub mod controller;

pub fn start(state: &mut State) -> Result<(), Box<dyn Error>> {
    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}
