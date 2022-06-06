use std::path::PathBuf;

use crate::repo::Repo;
use clap::Args;

#[derive(Args, Debug)]
pub struct Init {
    /// Where to create the repository.
    #[clap(default_value_t = String::from("."))]
    pub path: String,
}

pub fn cmd_init(opts: &Init) -> Result<(), String> {
    let repo: Repo = Repo::new(&PathBuf::from(&opts.path))?;
    let path: PathBuf = repo.work_tree.canonicalize().unwrap();
    println!("Initialized empty Git repository in {}", path.display());
    Ok(())
}
