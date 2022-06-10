use clap::Args;

use crate::{object::refs, repo::Repo};

#[derive(Args, Debug)]
pub struct ShowRef;

/// Print out a list of hash, path pairs of all the refs in this repository.
pub fn cmd_show_ref() -> Result<(), String> {
  let repo: Repo = Repo::default();
  let refs = refs::collect(&repo, None);
  for (k, v) in refs.iter() {
    println!("{} {}", v, k)
  }
  Ok(())
}
