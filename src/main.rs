pub mod cli;
pub mod repo;

use self::cli::{Arguments, Commands};
use clap::Parser;

use crate::cli::add::cmd_add;
use crate::cli::cat_file::cmd_cat_file;
use crate::cli::checkout::cmd_checkout;
use crate::cli::commit::cmd_commit;
use crate::cli::hash_object::cmd_hash_object;
use crate::cli::init::cmd_init;
use crate::cli::log::cmd_log;
use crate::cli::merge::cmd_merge;
use crate::cli::rebase::cmd_rebase;
use crate::cli::rev_parse::cmd_rev_parse;
use crate::cli::rm::cmd_rm;
use crate::cli::show_ref::cmd_show_ref;
use crate::cli::show_tree::cmd_show_tree;
use crate::cli::tag::cmd_tag;

fn main() {
    let args: Arguments = Arguments::parse();

    println!("args: {:?}", args);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.command {
        Commands::Add(_) => cmd_add(),
        Commands::CatFile(_) => cmd_cat_file(),
        Commands::Checkout(_) => cmd_checkout(),
        Commands::Commit(_) => cmd_commit(),
        Commands::HashObject(_) => cmd_hash_object(),
        Commands::Init(_) => cmd_init(),
        Commands::Log(_) => cmd_log(),
        Commands::ShowTree(_) => cmd_show_tree(),
        Commands::Merge(_) => cmd_merge(),
        Commands::Rebase(_) => cmd_rebase(),
        Commands::RevParse(_) => cmd_rev_parse(),
        Commands::Rm(_) => cmd_rm(),
        Commands::ShowRef(_) => cmd_show_ref(),
        Commands::Tag(_) => cmd_tag(),
    }
}
