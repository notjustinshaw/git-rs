use clap::Args;

#[derive(Args, Debug)]
pub struct Log {
  pub name: Option<String>,
}

pub fn cmd_log() -> Result<(), String> {
  todo!("cmd_log");
}
