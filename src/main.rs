use gitit;

use std::error::Error;
use gitit::controller::{init, state::State};

fn main() -> Result<(), Box<dyn Error>> {
    Ok(gitit::start()?)
}
