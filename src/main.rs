pub mod cli;
mod crypto;
mod object;
pub mod repo;

use self::cli::{Arguments, Command};
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
  // multiplex the command line args
  let args: Arguments = Arguments::parse();
  let response: Result<(), String> = match &args.command {
    Command::Add(_) => cmd_add(),
    Command::CatFile(opts) => cmd_cat_file(opts),
    Command::Checkout(_) => cmd_checkout(),
    Command::Commit(_) => cmd_commit(),
    Command::HashObject(opts) => cmd_hash_object(opts),
    Command::Init(opts) => cmd_init(opts),
    Command::Log(opts) => cmd_log(opts),
    Command::ShowTree(_) => cmd_show_tree(),
    Command::Merge(_) => cmd_merge(),
    Command::Rebase(_) => cmd_rebase(),
    Command::RevParse(_) => cmd_rev_parse(),
    Command::Rm(_) => cmd_rm(),
    Command::ShowRef(_) => cmd_show_ref(),
    Command::Tag(_) => cmd_tag(),
  };

  // handle the response type if it errored out
  if let Some(err) = response.err() {
    println!("fatal: {}", err);
  }
}
