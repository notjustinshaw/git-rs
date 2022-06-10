use std::any::Any;

use crate::repo::Repo;

pub trait Serializable {
  fn serialize(&self) -> &[u8];
  fn deserialize(&mut self, data: &[u8]);
  fn format(&self) -> &String;
  fn repo(&self) -> &Repo;
}

pub trait Unbox {
  fn unbox<T: Any>(&self) -> Result<&T, String>;
}

impl Unbox for Box<dyn Serializable> {
  fn unbox<T: Any>(&self) -> Result<&T, String> {
    let upcast_self: &dyn Any = self;
    match upcast_self.downcast_ref::<T>() {
      Some(cmt) => Ok(cmt),
      None => Err(format!("downcast to commit failed")),
    }
  }
}
