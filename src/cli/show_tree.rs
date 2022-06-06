use clap::Args;

#[derive(Args, Debug)]
pub struct ShowTree {
    pub name: Option<String>,
}

pub fn cmd_show_tree() -> Result<(), String> {
    todo!("cmd_show_tree");
}
