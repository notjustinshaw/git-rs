use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{
  fs::{self, File},
  io::Read,
  path::Path,
  process::Command,
};
use tempdir::TempDir; // Used for writing assertions

#[test]
fn test_init() -> Result<(), Box<dyn std::error::Error>> {
  init_template(None)?;
  init_template(Some("."))?;
  init_template(Some("mypath"))?;
  init_template(Some("with-dashes"))?;
  init_template(Some("with_underscores"))?;
  init_template(Some("path/to/repo"))?;
  Ok(())
}

fn init_template(path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
  // Load the git-rs binary
  let mut cmd = Command::cargo_bin("git-rs")?;

  // Create a new temporary directory
  let temp_dir = TempDir::new("gitrs")?;
  let canonical_path = temp_dir.path().canonicalize().unwrap();
  assert!(is_empty_directory(&canonical_path));

  // set the current directory and run `git-rs init`
  cmd.current_dir(&canonical_path);
  cmd.arg("init");
  if let Some(p) = path {
    cmd.arg(p);
  }

  // verify the module works as expected
  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Initialized empty Git repository"));
  assert!(!is_empty_directory(&canonical_path));

  // verify the `.git` directory is correct
  let git_dir = if let Some(p) = path {
    canonical_path.join(p).join(".git")
  } else {
    canonical_path.join(".git")
  };
  assert!(verify_file_matches(
    &git_dir.join("HEAD"),
    "ref: refs/heads/master\n"
  ));
  assert!(verify_file_matches(
    &git_dir.join("config"),
    "[core]\n\
    repositoryformatversion=0\n\
    filemode=false\n\
    bare=false\n"
  ));
  assert!(verify_file_matches(
    &git_dir.join("description"),
    "Unnamed repository; edit this file 'description' to name the repository.\n"
  ));
  assert!(is_empty_directory(&git_dir.join("objects")));
  assert!(is_empty_directory(&git_dir.join("branches")));
  assert!(is_empty_directory(&git_dir.join("refs").join("heads")));
  assert!(is_empty_directory(&git_dir.join("refs").join("tags")));

  Ok(())
}

fn is_empty_directory(dir: &Path) -> bool {
  match fs::read_dir(dir) {
    Err(msg) => panic!("fatal ({:?}): {}", dir, msg),
    Ok(iter) => iter.count() == 0,
  }
}

fn verify_file_matches(path: &Path, expected: &str) -> bool {
  match File::open(path) {
    Err(open_msg) => panic!("fatal ({:?}): {}", path, open_msg),
    Ok(mut f) => {
      let mut buf: String = String::default();
      match f.read_to_string(&mut buf) {
        Err(read_msg) => panic!("fatal ({:?}): {}", path, read_msg),
        Ok(_) => assert_eq!(buf, expected.to_owned()),
      }
    }
  }
  true
}
