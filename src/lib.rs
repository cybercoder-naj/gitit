use std::error::Error;
use crate::cache::Cache;
use crate::controller::init;
use crate::controller::state::State;

pub mod terminal;
pub mod controller;
pub mod cache;

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut state = State::new();
    let mut cache = Cache::new();
    init(&mut state);

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, &mut state, &mut cache)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}