use std::collections::HashSet;

use clap::Args;
use colored::Colorize;
use indexmap::IndexMap;

use crate::{
  object::{commit::Commit, find_object, read},
  repo::Repo,
};

/// Display history of a given commit.
///
/// Starting at the given commit (or HEAD by default) this command will show
/// the history of changes for a particular commit.
///
/// # Example
/// ```bash
/// $ git log
/// ```
#[derive(Args, Debug)]
pub struct Log {
  /// The commit to start at.
  #[clap(default_value_t = String::from("HEAD"))]
  pub commit: String,
}

pub fn cmd_log(opts: &Log) -> Result<(), String> {
  let repo: Repo = Repo::default();
  let mut seen: HashSet<String> = HashSet::default();
  print_commit(
    repo.clone(),
    find_object(repo.clone(), &opts.commit, None, false).to_string(),
    &mut seen,
  )?;
  Ok(())
}

/// Walks through the commit tree and prints out each commit.
fn print_commit(repo: Repo, hash: String, seen: &mut HashSet<String>) -> Result<(), String> {
  if seen.contains(&hash) {
    // already added commit, nothing to do!
    return Ok(());
  }
  seen.insert(hash.to_owned());

  let commit_object = read(repo.clone(), &hash, Some("commit"))?;
  assert!(commit_object.get_format().eq("commit"));
  let commit: &Commit = match commit_object.as_any().downcast_ref::<Commit>() {
    Some(cmt) => cmt,
    None => return Err(format!("downcast to commit failed")),
  };

  let map: &IndexMap<String, String> = &commit.gob.map;
  if !map.contains_key("parent") {
    // base case - the initial commit has no parent
    return Ok(());
  }

  let parents: Vec<&str> = map.get("parent").unwrap().split(' ').collect();
  for parent in parents {
    for key in map.keys() {
      match key.as_str() {
        "tree" => println!("commit {}", map.get("tree").unwrap().yellow()),
        "author" => println!("Author: {}", map.get("author").unwrap()),
        "" => println!("\n    {}", map.get("").unwrap()),
        _ => continue,
      }
    }
    print_commit(repo.clone(), parent.to_string(), seen)?; // do without clone?
  }

  Ok(())
}
