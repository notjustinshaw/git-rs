use clap::Args;

#[derive(Args, Debug)]
pub struct RevParse {
  pub name: Option<String>,
}

pub fn cmd_rev_parse() {
  todo!("cmd_rev_parse");
}
