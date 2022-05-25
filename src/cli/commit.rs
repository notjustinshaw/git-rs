use clap::Args;

#[derive(Args, Debug)]
pub struct Commit {
    pub name: Option<String>,
}

pub fn cmd_commit() {
    todo!("cmd_commit");
}
