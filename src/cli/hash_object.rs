use std::{fs, path::PathBuf, str::FromStr};

use clap::Args;

use crate::repo::Repo;

#[derive(Args, Debug)]
pub struct HashObject {
    /// The type of the object.
    #[clap(name = "TYPE")]
    pub typename: Option<String>,

    /// Whether to write object to directory, defaults to false.
    #[clap(short)]
    pub write: bool,

    /// The file to hash.
    pub file: String,
}

/// Computes the hash of a file as an object.
///
/// If the `-w` flag is passed, writes the object to the git directory at the
/// path corresponding to its hash. If no flag
pub fn cmd_hash_object(opts: &HashObject) -> Result<(), String> {
    let repo: Option<Repo> = match &opts.write {
        true => Repo::find_repo(&PathBuf::from("."), true)?,
        false => None,
    };

    if let Ok(file) = fs::read(PathBuf::from_str(&opts.file).unwrap()) {
        println!("{:?}", file);
    } else {
        println!("fatal: object not found");
    }
    Ok(())
}
