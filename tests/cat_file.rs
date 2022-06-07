use assert_cmd::prelude::*;
use hex_literal::hex;
use predicates::prelude::*;
use std::{
  fs::{self, File},
  io::Write,
  process::Command,
};
use tempdir::TempDir;

#[test]
fn test_cat_file() -> Result<(), Box<dyn std::error::Error>> {
  cat_file_template(
    "blob",
    "3b18e512dba79e4c8300dd08aeb37f8e728b8dad",
    &hex!("78014bcac94f52303462c848cdc9c95728cf2fca49e1020044110689"),
    "hello world\n",
  )?;
  Ok(())
}

fn cat_file_template(
  obj: &str,
  hash: &str,
  compressed_data: &[u8],
  plaintext_data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  // Load the git-rs binary
  let mut init_cmd = Command::cargo_bin("git-rs")?;
  let mut cat_cmd = Command::cargo_bin("git-rs")?;

  // Create a new temporary directory
  let temp_dir = TempDir::new("gitrs")?;
  let canonical_path = temp_dir.path().canonicalize().unwrap();

  // set the current directory and run `git-rs init`
  init_cmd.current_dir(&canonical_path);
  init_cmd.arg("init");
  init_cmd.output()?;

  // set the current directory and run `git-rs cat-file`
  cat_cmd.current_dir(&canonical_path);
  cat_cmd.arg("cat-file").arg(obj).arg(hash);

  // Add the object file
  let dir_path = &canonical_path.join(".git").join("objects").join(&hash[..2]);
  fs::create_dir(&dir_path)?;
  let mut f = File::create(&dir_path.join(&hash[2..]))?;
  f.write(compressed_data)?;
  f.flush()?;

  // verify the module works as expected
  cat_cmd
    .assert()
    .success()
    .stdout(predicate::str::contains(plaintext_data));

  Ok(())
}
