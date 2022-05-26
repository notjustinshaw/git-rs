mod blob;
mod serializable;

use crate::crypto;
use crate::object::blob::Blob;
use crate::object::serializable::Serializable;
use crate::repo::{repo_file, Repo};
use std::fs::{self, File};
use std::io::prelude::*;

/// A git object.
///
/// In git, objects are a generic structure used for a lot of various things. At
/// its core, an object is a compressed file in the `.git` directory whose path
/// is determined by its contents. The path is computed using the SHA-1 hash of
/// the payload split into the first 2 bytes which represent the object's
/// directory followed by the last 30 bytes which represent the filename.
///
/// The path to the object `e673d1b7eaa0aa01b5bc2442d570a765bdaae751` would be
/// `.git/objects/e6/73d1b7eaa0aa01b5bc2442d570a765bdaae751`. Inside that file
/// would be a zlib compressed payload prepended with an object header.
///
/// The first 48 bytes of a commit object might look like this:
///
/// ```text
/// 00000000  63 6f 6d 6d 69 74 20 31  30 38 36 00 74 72 65 65  |commit 1086.tree|
/// 00000010  20 32 39 66 66 31 36 63  39 63 31 34 65 32 36 35  | 29ff16c9c14e265|
/// 00000020  32 62 32 32 66 38 62 37  38 62 62 30 38 61 35 61  |2b22f8b78bb08a5a|
/// ```
///
/// An object starts with a header that specifies its type: `blob`, `commit`,
/// `tag` or `tree`. This header is followed by an ASCII space (0x20), then the
/// size of the object in bytes as an ASCII number, then null (0x00) (the null
/// byte), then the contents of the object.
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
    type ImplType = Self;

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

pub enum GitObject {
    Blob(Blob),
    Object(Object),
}

/// Reads object object_id from the repository repo and returns an object
/// whose exact type depends on the object read from memory.
pub fn read(repo: Repo, hash: &str) -> Result<GitObject, String> {
    let directories = ["objects", &hash[0..2], &hash[2..]];
    let path = repo_file(&repo.git_dir, &directories, false);
    if let Ok(file) = fs::read(path.unwrap()) {
        let raw = crypto::decompress(&file)?;

        // Read the object type
        let first_space: usize = raw.find(' ').unwrap();
        let object_type: &str = &raw[0..first_space];

        // Read and validate the object size
        let null_byte: usize = raw[first_space..].find('\0').unwrap();
        let object_size: usize = raw[first_space..null_byte].parse::<usize>().unwrap();

        if object_size != raw.len() - null_byte - 1 {
            return Err("Object size does not match the size of the raw data.".to_string());
        }

        match object_type {
            "blob" => Ok(GitObject::Blob(Blob::new(repo, &raw[null_byte + 1..]))),
            "commit" => Ok(GitObject::Object(Object::new(repo, "commit"))),
            "tag" => Ok(GitObject::Object(Object::new(repo, "tag"))),
            "tree" => Ok(GitObject::Object(Object::new(repo, "tree"))),
            _ => Err("Object type not supported.".to_string()),
        }
    } else {
        Err("Object not found.".to_string())
    }
}

/// Writes an object to the repository.
///
/// The object is written to the repository that the object represents. If the
/// dry_run flag is set to true, the hash will be calculated but not written
/// to the directory.
pub fn write<T: Serializable>(object: &T, dry_run: bool) -> Result<String, String> {
    let payload = object.serialize();
    let header = format!("{} {}\0", object.get_format(), payload.len());
    let data = [header.as_bytes(), payload].concat();
    let hash = crypto::sha_1(&data);

    if !dry_run {
        let directories = ["objects", &hash[0..2], &hash[2..]];
        let path = repo_file(&object.get_repo().git_dir, &directories, true);
        let mut file = File::create(path.unwrap()).unwrap();
        let compressed_data = crypto::compress(&data)?;
        file.write_all(&compressed_data[..]).unwrap();
    }
    return Ok(hash);
}

fn find_object<'a>(_repo: Repo, name: &'a str, _type: Option<&str>, _follow: bool) -> &'a str {
    return name;
}
