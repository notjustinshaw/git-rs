use std::ops::Deref;

use crate::repo::Repo;

use super::{mail_map::MailMap, serializable::Serializable};

pub struct Commit {
  format: String,
  map: MailMap,
  repo: Repo,
}

impl Commit {
  pub fn new(repo: Repo, data: &[u8]) -> Self {
    let mut new_commit: Self = Self {
      format: String::from("commit"),
      map: MailMap::new(),
      repo,
    };
    new_commit.map.from_bytes(data, 0);
    return new_commit;
  }
}

impl Deref for Commit {
  type Target = MailMap;

  fn deref(&self) -> &Self::Target {
    &self.map
  }
}

impl Serializable for Commit {
  fn serialize(&self) -> &[u8] {
    self.map.to_bytes()
  }
  
  fn deserialize(&mut self, data: &[u8]) {
    self.map.from_bytes(data, 0)
  }

  fn format(&self) -> &String {
    &self.format
  }

  fn repo(&self) -> &Repo {
    &self.repo
  }
}
