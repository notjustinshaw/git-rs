use clap::Args;

#[derive(Args, Debug)]
pub struct Add {
    pub name: Option<String>,
}

pub fn cmd_add() {
    todo!("cmd_add");
}