use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::*;
use std::process::Command;

lazy_static! {
    static ref BIN_NAME: String = env!("CARGO_PKG_NAME").to_string();
}

#[test]
fn argument_does_not_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME.to_string())?;
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Found argument 'foobar' which wasn't expected,",
    ));

    Ok(())
}

use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn install_command_() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "name: random python project name
version: '0.1.4'
dependencies:"
    )?;

    let mut cmd = Command::cargo_bin(BIN_NAME.to_string())?;
    cmd.arg("--dry-run")
        .arg("-c")
        .arg(file.path())
        .arg("install");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY-RUN: what would be installed"));

    Ok(())
}
