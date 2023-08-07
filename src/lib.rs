use std::error::Error;

mod terminal;
mod controller;
mod utils;

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}
