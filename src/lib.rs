use std::error::Error;
use controller::init;
use controller::state::State;

mod terminal;
pub mod controller;

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut state = State::new();
    init(&mut state);

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, &mut state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}
