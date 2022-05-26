use crate::repo::Repo;

use super::{serializable::Serializable, Object};

pub struct Blob {
    pub object: Object,
    pub data: Vec<u8>,
}

impl Blob {
    pub fn new(repo: Repo, data: &str) -> Self {
        Self {
            object: Object::new(repo, "blob"),
            data: data.as_bytes().to_vec(),
        }
    }
}

impl Serializable for Blob {
    type ImplType = Self;

    fn serialize(&self) -> &[u8] {
        return &self.data;
    }

    fn deserialize(&mut self, data: &str) {
        self.data = data.as_bytes().to_vec();
    }

    fn get_format(&self) -> &str {
        self.object.get_format()
    }

    fn get_repo(&self) -> &Repo {
        &self.object.get_repo()
    }
}
