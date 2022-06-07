use clap::Args;

/// Display history of a given commit.
///
/// Starting at the given commit (or HEAD by default) this command will show
/// the history of changes for a particular commit.
///
/// # Example
/// ```bash
/// $ git log
/// ```
#[derive(Args, Debug)]
pub struct Log {
  /// The commit to start at
  pub commit: Option<String>,
}

pub fn cmd_log() -> Result<(), String> {
  todo!("cmd_log");
}
