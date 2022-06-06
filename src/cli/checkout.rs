use clap::Args;

#[derive(Args, Debug)]
pub struct Checkout {
    pub name: Option<String>,
}

pub fn cmd_checkout() -> Result<(), String> {
    todo!("cmd_checkout");
}
