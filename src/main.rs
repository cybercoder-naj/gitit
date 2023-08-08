use gitit::{
    controller::{init, state::State},
    terminal
};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::new();
    init(&mut state);

    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal, &mut state)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}