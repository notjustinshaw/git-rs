use clap::Args;

#[derive(Args, Debug)]
pub struct RevParse {
    pub name: Option<String>,
}

pub fn cmd_rev_parse() -> Result<(), String> {
    todo!("cmd_rev_parse");
}
