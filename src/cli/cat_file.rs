use std::path::PathBuf;

use clap::Args;

use crate::{object::read, repo::Repo};

#[derive(Args, Debug)]
pub struct CatFile {
  /// Specify the type.
  #[clap(name = "TYPE")]
  pub typename: String,

  /// The object to display.
  pub object: String,
}

/// Prints a compressed object file.
///
/// Looks in the git directory for an object file with the given object hash. If
/// found, try to uncompress it and parse the payload data.
///
/// # Example
/// ```bash
/// $ git cat-file blob 00a534409c6fe1acb2cf24f17d101a4d0016c3f5
/// ```
pub fn cmd_cat_file(opts: &CatFile) -> Result<(), String> {
  if let Some(repo) = Repo::find_repo(&PathBuf::from("."), true)? {
    let gob = read(repo, &opts.object, &opts.typename)?;
    print!("{}", String::from_utf8_lossy(gob.serialize()).to_string());
    Ok(())
  } else {
    Err(format!("repository not found"))
  }
}
