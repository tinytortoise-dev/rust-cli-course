use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

fn run(args: &[&str], expected_file: &str) -> Result<()> {
  let expected = fs::read_to_string(expected_file)?;
  let output = Command::cargo_bin("echor")?
    .args(args)
    .output()
    .expect("fail");

  let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
  assert_eq!(stdout, expected);
  Ok(())
}

#[test]
fn dies_no_args() -> Result<()> {
  let mut cmd = Command::cargo_bin("echor")?;
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Usage"));
  Ok(())
}

#[test]
fn hello1() -> Result<()> {
  let outfile = "tests/expected/hello1.txt";
  let expected = fs::read_to_string(outfile)?;
  let mut cmd = Command::cargo_bin("echor")?;
  cmd
    .arg("Hello there")
    .assert()
    .success()
    .stdout(expected);
  Ok(())
}

#[test]
fn hello2() -> Result<()> {
  let expected = fs::read_to_string("tests/expected/hello2.txt")?;
  let mut cmd = Command::cargo_bin("echor")?;
  cmd
    .args(vec!["Hello", "there"])
    .assert()
    .success()
    .stdout(expected);
  Ok(())
}