use std::ops::Deref;

use crate::repo::Repo;

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
  format: String,
  map: MailMap,
  repo: Repo,
}

impl Tag {
  pub fn new(repo: Repo, data: &[u8]) -> Self {
    let mut new_tag: Self = Self {
      format: String::from("tag"),
      map: MailMap::new(),
      repo,
    };
    new_tag.map.parse_bytes(data, 0);
    new_tag
  }
}

impl Deref for Tag {
  type Target = MailMap;

  fn deref(&self) -> &Self::Target {
    &self.map
  }
}

impl Serializable for Tag {
  fn serialize(&self) -> &[u8] {
    self.map.to_bytes()
  }

  fn deserialize(&mut self, data: &[u8]) {
    self.map.parse_bytes(data, 0)
  }

  fn format(&self) -> &String {
    &self.format
  }

  fn repo(&self) -> &Repo {
    &self.repo
  }
}
