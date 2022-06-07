use crate::repo::Repo;

use super::{git_object::GitObject, serializable::Serializable, Object};

pub struct Commit {
  pub object: Object,
  pub gob: GitObject,
}

impl Commit {
  pub fn new(repo: Repo, _data: &str) -> Self {
    Self {
      object: Object::new(repo, "commit"),
      gob: GitObject::new(),
    }
  }
}

impl Serializable for Commit {
  fn serialize(&self) -> &[u8] {
    return &self.gob.to_bytes();
  }

  fn deserialize(&mut self, data: &str) {
    self.gob.from_bytes(data.as_bytes(), 0);
  }

  fn get_format(&self) -> &str {
    self.object.get_format()
  }

  fn get_repo(&self) -> &Repo {
    &self.object.get_repo()
  }
}
