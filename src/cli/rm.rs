use clap::Args;

#[derive(Args, Debug)]
pub struct Rm {
    pub name: Option<String>,
}

pub fn cmd_rm() -> Result<(), String> {
    todo!("cmd_rm");
}
