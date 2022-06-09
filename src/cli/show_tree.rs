use clap::Args;

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
  assert!(tree_object.get_format().eq("tree"));
  let tree: &Tree = match tree_object.as_any().downcast_ref::<Tree>() {
    Some(cmt) => cmt,
    None => return Err(format!("downcast to commit failed")),
  };

  for item in &tree.entries {
    println!(
      "{} {} {}\t{}",
      item.mode,
      read(repo.clone(), &item.hash, None).unwrap().get_format(),
      item.hash,
      item.path
    );
  }

  Ok(())
}
