use clap::Args;

#[derive(Args, Debug)]
pub struct Merge {
  pub name: Option<String>,
}

pub fn cmd_merge() -> Result<(), String> {
  todo!("cmd_merge");
}
