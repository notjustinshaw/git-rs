use clap::Args;

#[derive(Args, Debug)]
pub struct ShowRef {
  pub name: Option<String>,
}

pub fn cmd_show_ref() -> Result<(), String> {
  todo!("cmd_show_ref");
}
