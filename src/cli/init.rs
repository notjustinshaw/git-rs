use std::path::PathBuf;

use crate::repo::Repo;
use clap::Args;

#[derive(Args, Debug)]
pub struct Init {
    /// Where to create the repository.
    #[clap(default_value_t = String::from("."))]
    pub path: String,
}

pub fn cmd_init(opts: &Init) {
    match Repo::new(&PathBuf::from(&opts.path)) {
        Ok(repo) => {
            println!(
                "Initialized empty Git repository in {}",
                repo.work_tree.display()
            );
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
