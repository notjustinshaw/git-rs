use crate::repo::Repo;

use super::serializable::Serializable;

pub struct Blob {
  data: Vec<u8>,
  format: String,
  repo: Repo,
}

impl Blob {
  pub fn new(repo: Repo, data: &[u8]) -> Self {
    Self {
      data: data.to_vec(),
      format: String::from("blob"),
      repo,
    }
  }

  pub fn data(&self) -> &Vec<u8> {
    &self.data
  }
}

impl Serializable for Blob {
  fn serialize(&self) -> &[u8] {
    return &self.data;
  }

  fn deserialize(&mut self, data: &[u8]) {
    self.data = data.to_vec();
  }

  fn format(&self) -> &String {
    &self.format
  }

  fn repo(&self) -> &Repo {
    &self.repo
  }
}
