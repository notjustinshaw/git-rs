pub(crate) mod add;
pub(crate) mod cat_file;
pub(crate) mod checkout;
pub(crate) mod commit;
pub(crate) mod hash_object;
pub(crate) mod init;
pub(crate) mod log;
pub(crate) mod merge;
pub(crate) mod rebase;
pub(crate) mod rev_parse;
pub(crate) mod rm;
pub(crate) mod show_ref;
pub(crate) mod show_tree;
pub(crate) mod tag;

use add::Add;
use cat_file::CatFile;
use checkout::Checkout;
use clap::{Parser, Subcommand};
use commit::Commit;
use hash_object::HashObject;
use init::Init;
use log::Log;
use merge::Merge;
use rebase::Rebase;
use rev_parse::RevParse;
use rm::Rm;
use show_ref::ShowRef;
use show_tree::ShowTree;
use tag::Tag;

/// the rusty content tracker
///
/// git.rs is a an attempt at learning the git version control system by
/// building one from the ground up, in rust. The goal will be to write a
/// program (`git-rs`) with all the core git functionality in a way that will be
/// easily compatible with other git implementations.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add file contents to the index.
    Add(Add),

    /// Provide content or type and size information for repository objects.
    CatFile(CatFile),

    /// Switch branches or restore working tree files.
    Checkout(Checkout),

    /// Record changes to the repository.
    Commit(Commit),

    /// Compute object ID and optionally creates a blob from a file.
    HashObject(HashObject),

    /// Create an empty Git repository or reinitialize an existing one.
    Init(Init),

    /// Show commit logs.
    Log(Log),

    /// List the contents of a tree object.
    ShowTree(ShowTree),

    /// Join two or more development histories together.
    Merge(Merge),

    /// Reapply commits on top of another base tip.
    Rebase(Rebase),

    /// Pick out and massage parameters.
    RevParse(RevParse),

    /// Remove files from the working tree and from the index.
    Rm(Rm),

    /// List references in a local repository.
    ShowRef(ShowRef),

    /// Create, list, delete or verify a tag object signed with GPG.
    Tag(Tag),
}
