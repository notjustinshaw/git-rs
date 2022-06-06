use clap::Args;

#[derive(Args, Debug)]
pub struct Tag {
    pub name: Option<String>,
}

pub fn cmd_tag() -> Result<(), String> {
    todo!("cmd_tag");
}
