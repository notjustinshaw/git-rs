use clap::Args;

#[derive(Args, Debug)]
pub struct CatFile {
    pub name: Option<String>,
}

pub fn cmd_cat_file() {
    todo!("cmd_cat_file");
}
