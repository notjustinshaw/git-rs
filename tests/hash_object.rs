use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{fs::File, io::Write, process::Command};
use tempdir::TempDir;

#[test]
fn test_hash_object() -> Result<(), Box<dyn std::error::Error>> {
  hash_object_template(
    "hello.txt",
    Some("blob"),
    false,
    "3b18e512dba79e4c8300dd08aeb37f8e728b8dad",
    "hello world\n",
  )?;
  hash_object_template(
    "hello.txt",
    None,
    false,
    "3b18e512dba79e4c8300dd08aeb37f8e728b8dad",
    "hello world\n",
  )?;
  hash_object_template(
    "hello.txt",
    Some("blob"),
    true,
    "3b18e512dba79e4c8300dd08aeb37f8e728b8dad",
    "hello world\n",
  )?;
  Ok(())
}

fn hash_object_template(
  filename: &str,
  obj: Option<&str>,
  write: bool,
  hash: &str,
  plaintext_data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  // Load the git-rs binary
  let mut init_cmd = Command::cargo_bin("git-rs")?;
  let mut hash_cmd = Command::cargo_bin("git-rs")?;

  // Create a new temporary directory
  let temp_dir = TempDir::new("gitrs")?;
  let canonical_path = temp_dir.path().canonicalize().unwrap();

  // set the current directory and run `git-rs init`
  init_cmd.current_dir(&canonical_path);
  init_cmd.arg("init");
  init_cmd.output()?;

  // set the current directory and run `git-rs hash-object`
  hash_cmd.current_dir(&canonical_path);
  hash_cmd.arg("hash-object").arg(filename);
  if obj.is_some() {
    hash_cmd.arg(obj.unwrap());
  }
  if write {
    hash_cmd.arg("--write");
  }

  // Add the file to be hash-object'ed
  let mut f = File::create(&canonical_path.join(filename))?;
  f.write(plaintext_data.as_bytes())?;
  f.flush()?;

  // verify the module works as expected
  hash_cmd
    .assert()
    .success()
    .stdout(predicate::str::contains(hash));

  // set command config
  if write && obj.is_some() {
    let mut cat_cmd = Command::cargo_bin("git-rs")?;

    cat_cmd.current_dir(&canonical_path);
    cat_cmd.arg("cat-file").arg(obj.unwrap()).arg(hash);

    // verify the module works as expected
    cat_cmd
      .assert()
      .success()
      .stdout(predicate::str::contains(plaintext_data));
  }

  Ok(())
}
