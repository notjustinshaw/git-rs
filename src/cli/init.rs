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
    Repo::new(&PathBuf::from(&opts.path));
}
