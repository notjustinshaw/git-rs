use clap::Args;

use crate::object::serializable::Unbox;
use crate::object::tree::Tree;
use crate::{object::read, repo::Repo};

/// Print the contents of a tree object.
#[derive(Args, Debug)]
pub struct ShowTree {
  /// The object to show.
  pub object: String,
}

pub fn cmd_show_tree(opts: &ShowTree) -> Result<(), String> {
  let repo: Repo = Repo::default();
  let tree_object = read(repo.clone(), &opts.object, Some("tree"))?;
  assert!(tree_object.format().eq("tree"));
  let tree: &Tree = tree_object.unbox::<Tree>()?;

  for item in tree.entries() {
    println!(
      "{} {} {}\t{}",
      item.mode,
      read(repo.clone(), &item.hash, None).unwrap().format(),
      item.hash,
      item.path
    );
  }

  Ok(())
}
