use gitit;

use std::error::Error;
use gitit::controller::{init, state::State};

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::new();
    init(&mut state);

    Ok(gitit::start(&mut state)?)
}
