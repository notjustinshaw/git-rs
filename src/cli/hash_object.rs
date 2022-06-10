use std::{fs, path::PathBuf, str::FromStr};

use clap::Args;

use crate::object::blob::Blob;
use crate::object::commit::Commit;
use crate::object::serializable::Serializable;
use crate::object::tag::Tag;
use crate::object::tree::Tree;
use crate::object::write;
use crate::repo::Repo;

/// Hashes a file into a loose object.
///
/// Converts a file into object form. The file contents are compressed and
/// stored on the filesystem with an object header. The header contains some
/// simple information about the object such as the object type and its size.
///
/// ### Example
/// ```bash
/// $ git hash-object hello.txt
/// 3b18e512dba79e4c8300dd08aeb37f8e728b8dad
/// ```
///
/// ## What about packfiles?
/// Packfiles are another way that `git` stores information about a repository.
/// They're more efficient and also more complicated. Essentially, packfiles
/// are a combination of loose objects (similar to a `tar`) with the major
/// exception that some of them are only diffs (transformations from one object
/// into another).
///
/// ## How packfiles work
/// A packfile is stored in `.git/objects/pack` in a file with a `.pack`
/// extension. The packfile has an accompanying index file with the same name
/// but with a `.idx` extension.
///
/// You can convert a packfile into loose objects by first moving it out of the
/// `.git/objects/pack` directory and then unpacking it.
///
/// ### Example
/// ```bash
/// $ mv .git/objects/pack/pack-d9ef004d4ca729287f12aaaacf36fee39baa7c9d.pack .
/// ```
/// You can ignore the `.idx` file. Then, you'll want to unpack the packfile
/// using `git-unpack-objects`:
/// ```bash
/// $ cat pack-d9ef004d4ca729287f12aaaacf36fee39baa7c9d.pack | git unpack-objects
/// ```

#[derive(Args, Debug)]
pub struct HashObject {
  /// The file to hash.
  pub file: String,

  /// Write the object into the object database.
  #[clap(short, long)]
  pub write: bool,

  /// The object type.
  #[clap(name = "TYPE", default_value_t = String::from("blob"))]
  pub typename: String,
}

/// Computes the hash of a file as an object.
///
/// If the `-w` flag is passed, writes the object to the git directory at the
/// path corresponding to its hash and prints its hash. If not write flag is
/// given, only prints the hash.
pub fn cmd_hash_object(opts: &HashObject) -> Result<(), String> {
  let repo: Repo = Repo::default();
  let path: PathBuf = PathBuf::from_str(&opts.file).unwrap();
  if let Ok(file) = fs::read(path) {
    let obj: Box<dyn Serializable> = match opts.typename.as_str() {
      "blob" => Box::new(Blob::new(repo, &file)),
      "commit" => Box::new(Commit::new(repo, &file)),
      "tag" => Box::new(Tag::new(repo, &file)),
      "tree" => Box::new(Tree::new(repo, &file)),
      _ => return Err(format!("unsupported type \"{}\"", opts.typename)),
    };
    println!("{}", write(&obj, !opts.write)?);
    Ok(())
  } else {
    Err(format!("object not found"))
  }
}
