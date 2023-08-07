use gitit;

use std::error::Error;
use gitit::controller::init;
use gitit::controller::state::GitState;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = GitState::new();
    init(&mut state);

    Ok(gitit::start(&mut state)?)
}
