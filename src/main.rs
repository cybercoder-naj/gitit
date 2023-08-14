mod terminal;
mod controller;
mod cache;

use std::error::Error;
use controller::{state::State, init};
use crate::cache::Cache;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::new();
    let mut cache = Cache::new();
    init(&mut state);

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, &mut state, &mut cache)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}