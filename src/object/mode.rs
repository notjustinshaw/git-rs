use std::convert::TryFrom;
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
  Normal = 100644,
  Directory = 40000,
  Executable = 100755,
  Symbolic = 120000,
}

impl Display for Mode {
  /// Pads the mode to six digits.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:0>6}", *self as usize)
  }
}

impl TryFrom<usize> for Mode {
  type Error = ();

  fn try_from(value: usize) -> Result<Self, Self::Error> {
    match value {
      x if x == Mode::Normal as usize => Ok(Mode::Normal),
      x if x == Mode::Directory as usize => Ok(Mode::Directory),
      x if x == Mode::Executable as usize => Ok(Mode::Executable),
      x if x == Mode::Symbolic as usize => Ok(Mode::Symbolic),
      _ => Err(()),
    }
  }
}
