use crate::repo::Repo;

pub trait Serializable {
  fn serialize(&self) -> &[u8];
  fn deserialize(&mut self, data: &str);
  fn get_format(&self) -> &str;
  fn get_repo(&self) -> &Repo;
}
