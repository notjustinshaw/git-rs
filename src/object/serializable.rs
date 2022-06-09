use std::any::Any;

use crate::repo::Repo;

pub trait Serializable {
  fn serialize(&self) -> &[u8];
  fn deserialize(&mut self, data: &[u8]);
  fn get_format(&self) -> &str;
  fn get_repo(&self) -> &Repo;
  fn as_any(&self) -> &dyn Any;
}

pub trait Unbox {
  fn unbox<T: Any>(&self) -> Result<&T, String>;
}

impl Unbox for Box<dyn Serializable> {
  fn unbox<T: Any>(&self) -> Result<&T, String> {
    match self.as_any().downcast_ref::<T>() {
      Some(cmt) => Ok(cmt),
      None => Err(format!("downcast to commit failed")),
    }
  }
}
