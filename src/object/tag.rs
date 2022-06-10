use crate::object::Object;
use crate::repo::Repo;
use std::any::Any;

use super::mail_map::MailMap;
use super::serializable::Serializable;

/// A git tag.
///
/// A tag is a git object that stores some metadata along with a message in a
/// plaintext file in MailMap form. The tag points at a commit and includes
/// some information about the tag and the pointed-to object.
///
/// A sample tag object might look like this:
/// ```bash
/// object 817abab1dd32cdf6ca40f4d75242064479817141
/// type commit
/// tag justins-special-tag-v2
/// tagger Justin Shaw <realjustinshaw@gmail.com> 1654899880 -0700
///
/// hello world!
/// ```
pub struct Tag {
  pub object: Object,
  pub map: MailMap,
}

impl Tag {
  pub fn new(repo: Repo, data: &[u8]) -> Self {
    let mut map = MailMap::new();
    map.from_bytes(data, 0);
    Self {
      object: Object::new(repo, "tag"),
      map,
    }
  }
}

impl Serializable for Tag {
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
