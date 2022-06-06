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

pub fn cmd_cat_file(opts: &CatFile) {
    let repo = match Repo::find_repo(&PathBuf::from("."), true) {
        Ok(repo) => repo,
        Err(error) => {
            println!("Repository not found.");
            println!("{}", error.to_string());
            return;
        }
    };
    match read(repo.unwrap(), &opts.object, &opts.typename) {
        Ok(gob) => {
            println!("{}", String::from_utf8_lossy(gob.serialize()).to_string());
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
