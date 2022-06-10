use clap::Args;
use std::fs::write;
use std::path::Path;

use crate::{
  object::{blob::Blob, commit::Commit, read, serializable::Unbox, tree::Tree},
  repo::Repo,
};

#[derive(Args, Debug)]
pub struct Checkout {
  /// The commit or tree to checkout.
  pub object: String,

  /// The EMPTY directory to checkout on.
  pub path: String,
}

pub fn cmd_checkout(opts: &Checkout) -> Result<(), String> {
  let repo: Repo = Repo::default();

  // Parse the commit into a commit object.
  let mut object = read(repo.clone(), &opts.object, None)?;

  // Parse the commit object into a tree.
  if object.format().eq("commit") {
    let commit = object.unbox::<Commit>()?;
    let tree_hash = commit.map.get("tree").unwrap();
    object = read(repo.clone(), tree_hash, Some("tree"))?;
  }

  let tree = object.unbox::<Tree>()?;

  // Verify the path is an empty directory. If it's not a directory, fail with
  // an Err. if it's not empty, fail with an Err. If some of the paths don't
  // exist yet, create them.
  let path = Path::new(&opts.path);
  // create the directory entries along the path if they do not exist
  if let Err(msg) = std::fs::create_dir_all(path) {
    return Err(format!("failed to create path {} ({})", &opts.path, msg));
  }
  if !path.is_dir() {
    return Err(format!("{} is not a directory", opts.path));
  }
  if !path.read_dir().unwrap().next().is_none() {
    return Err(format!("{} is not empty", opts.path));
  }

  tree_checkout(&repo, &tree, path)?;
  Ok(())
}

fn tree_checkout(repo: &Repo, tree: &Tree, path: &Path) -> Result<(), String> {
  for item in tree.entries() {
    let obj = read(repo.clone(), &item.hash, None)?;
    let dest = path.join(item.path.as_str());

    if obj.format().eq("tree") {
      if let Err(msg) = std::fs::create_dir_all(&dest) {
        return Err(format!("failed to create path {:?} ({})", &dest, msg));
      }
      let tree = obj.unbox::<Tree>()?;
      tree_checkout(repo, &tree, &dest)?;
    } else if obj.format().eq("blob") {
      if let Err(msg) = write(&dest, &obj.unbox::<Blob>()?.data()) {
        return Err(format!("failed to write file {:?} ({})", &dest, msg));
      }
    }
  }
  Ok(())
}
