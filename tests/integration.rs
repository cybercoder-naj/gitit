use assert_cmd::prelude::*; // Add methods on commands
 // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitit")?;

    cmd.assert();

    Ok(())
}
