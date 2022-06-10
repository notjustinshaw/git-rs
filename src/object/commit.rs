use std::any::Any;

use crate::repo::Repo;

use super::{serializable::Serializable, Object, mail_map::MailMap};

pub struct Commit {
  pub object: Object,
  pub map: MailMap,
}

impl Commit {
  pub fn new(repo: Repo, data: &[u8]) -> Self {
    let mut map = MailMap::new();
    map.from_bytes(data, 0);
    Self {
      object: Object::new(repo, "commit"),
      map,
    }
  }
}

impl Serializable for Commit {
  fn serialize(&self) -> &[u8] {
    return &self.map.to_bytes();
  }

  fn deserialize(&mut self, data: &[u8]) {
    self.map.from_bytes(data, 0);
  }

  fn get_format(&self) -> &str {
    self.object.get_format()
  }

  fn get_repo(&self) -> &Repo {
    &self.object.get_repo()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}
