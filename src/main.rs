use gitit;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(gitit::start()?)
}
