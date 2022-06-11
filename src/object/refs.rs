use crate::repo::{repo_dir, Repo};
use std::collections::BTreeMap;
use std::{
  fs,
  path::{Path, PathBuf},
};

/// Resolves a ref path to an object hash.
///
/// A ref associates a name to a particular git object. Refs can either be
/// direct or indirect. A direct ref is a file where the name of the file is the
/// name of the ref and the data in the file is the SHA-1 hash of the object
/// that the ref refers to. An indirect ref is like a direct ref, except instead
/// of storing the hash of the object directly in the file, we store a string
/// which represents the path to another ref (which might, in turn, point at
/// another indirect ref). Indirect refs must be recursively resolves.
pub fn resolve(repo: &Repo, refr: &Path) -> Result<String, String> {
  let path: PathBuf = if refr.starts_with(&repo.git_dir) {
    PathBuf::from(refr)
  } else {
    repo.git_dir.join(refr)
  };
  match fs::read(&path) {
    Ok(data) => {
      if data.starts_with("ref: ".as_bytes()) {
        // indirect ref stores a plain-text path to another ref (ie. recursive)
        match String::from_utf8(data[5..data.len() - 1].to_vec()) {
          Ok(next_ref) => resolve(repo, &Path::new(&next_ref)),
          Err(msg) => Err(format!("unable to parse ref ({})", msg)),
        }
      } else {
        // direct ref is an utf8-encoded string of the object hash
        let object_hash = data[..data.len() - 1].to_vec(); // trim trailing \n
        return Ok(String::from_utf8(object_hash).unwrap());
      }
    }
    Err(msg) => Err(format!("{} {}", &path.to_string_lossy(), msg)),
  }
}

/// Collects refs and returns them as an ordered dictionary.
///
/// Starts in the `.git/refs` directory and recursively builds up a map between
/// paths and ref hashes. The paths are stored in a prefixed form starting with
/// `refs/` and each ref is resolved into a hash before being stored.
pub fn collect(repo: &Repo, path: Option<&Path>) -> BTreeMap<String, String> {
  let default_path = repo_dir(&repo.git_dir, &["refs"], true).unwrap();
  let path = path.unwrap_or(&default_path);
  let mut map = BTreeMap::new();
  for entry in path.read_dir().expect("unable to read dir") {
    if let Ok(entry) = entry {
      let entry_path = entry.path();
      let new_path = path.join(&entry_path);
      if entry.file_type().unwrap().is_dir() {
        // build a map of the sub-directory, then flatten result into this map
        let sub_map = collect(repo, Some(new_path.as_path()));
        for pair in sub_map {
          map.insert(pair.0, pair.1);
        }
      } else {
        // resolve this ref, store the path suffix and its object hash
        let path_suffix = entry_path.strip_prefix(&repo.git_dir);
        let filename = path_suffix.unwrap().to_string_lossy().into_owned();
        map.insert(filename, resolve(repo, new_path.as_path()).unwrap());
      }
    }
  }
  return map;
}
