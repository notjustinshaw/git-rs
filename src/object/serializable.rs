use std::any::Any;

use crate::repo::Repo;

pub trait Serializable {
  fn serialize(&self) -> &[u8];
  fn deserialize(&mut self, data: &[u8]);
  fn get_format(&self) -> &str;
  fn get_repo(&self) -> &Repo;
  fn as_any(&self) -> &dyn Any;
}
