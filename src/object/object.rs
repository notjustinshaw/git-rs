use crate::repo::Repo;

use super::serializable::Serializable;

pub struct Object {
    pub repo: Repo,
    pub format: String,
}

impl Object {
    pub fn new(repo: Repo, format: &str) -> Self {
        Self {
            repo,
            format: format.to_string(),
        }
    }
}

impl Serializable for Object {
    fn serialize(&self) -> &[u8] {
        unimplemented!();
    }

    fn deserialize(&mut self, _data: &str) {
        unimplemented!();
    }

    fn get_format(&self) -> &str {
        self.format.as_str()
    }

    fn get_repo(&self) -> &Repo {
        &self.repo
    }
}
