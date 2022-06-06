use clap::Args;

#[derive(Args, Debug)]
pub struct Rebase {
  pub name: Option<String>,
}

pub fn cmd_rebase() -> Result<(), String> {
  todo!("cmd_rebase");
}
