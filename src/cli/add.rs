use clap::Args;

#[derive(Args, Debug)]
pub struct Add {
  pub name: Option<String>,
}

pub fn cmd_add() -> Result<(), String> {
  todo!("cmd_add");
}
