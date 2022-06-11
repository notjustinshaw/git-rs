use std::{fs::File, io::Write, path::Path};

use clap::Args;

use crate::{
  object::refs,
  object::{
    self,
    mail_map::{self, MailMap},
    serializable::Serializable,
    tag::Tag as TagObject,
  },
  repo::{repo_dir, Repo},
};

/// List and create tags.
#[derive(Args, Debug)]
pub struct Tag {
  /// The name of the new tag.
  pub name: Option<String>,

  /// The object the new tag will point to.
  #[clap(default_value_t = String::from("HEAD"))]
  pub object: String,

  /// Creates an annotated tag.
  #[clap(short, long)]
  pub annotated: bool,
}

pub fn cmd_tag(opts: &Tag) -> Result<(), String> {
  let repo: Repo = Repo::default();
  if opts.name.is_none() {
    list_all_tags(&repo);
  } else if opts.annotated {
    let tag_name = &opts.name.as_ref().unwrap();
    let hash = create_annotated_tag(&repo, tag_name, &opts.object)?;
    create_simple_tag(&repo, tag_name, &hash);
  } else {
    create_simple_tag(&repo, &opts.name.as_ref().unwrap(), &opts.object);
  }
  Ok(())
}

/// Lists all tags in the given repository.
fn list_all_tags(repo: &Repo) {
  let path_buf = repo_dir(&repo.git_dir, &["refs", "tags"], true).unwrap();
  let refs = refs::collect(&repo, Some(path_buf.as_path()));
  let prefix = Path::new("refs/tags/");
  for k in refs.keys() {
    let tag_name = Path::new(k).strip_prefix(prefix).expect("strip prefix");
    println!("{}", tag_name.to_string_lossy())
  }
}

fn create_simple_tag(repo: &Repo, name: &String, object: &String) {
  let path = repo_dir(&repo.git_dir, &["refs", "tags"], true).unwrap();
  let mut file = File::create(path.join(name)).expect("create failed");
  if object.eq("HEAD") {
    let mut payload = refs::resolve(repo, &repo.git_dir.join("HEAD")).expect("resolve");
    payload.push_str("\n");
    file.write(payload.as_bytes()).expect("write failed");
  } else {
    let mut payload: String = String::from(object);
    payload.push('\n');
    file.write(payload.as_bytes()).expect("write failed");
  };
}

fn create_annotated_tag(repo: &Repo, name: &String, object: &String) -> Result<String, String> {
  // TODO: Create git objects with signature, name, email, and message (editor?)
  let mut mail_map: MailMap = MailMap::new();
  mail_map.map.insert("commit".to_owned(), object.to_owned());
  mail_map.map.insert("tag".to_owned(), name.to_owned());
  mail_map
    .map
    .insert("".to_owned(), "\n".to_owned());
  let payload = mail_map::map_to_bytes(&mail_map.map);
  let new_tag: Box<dyn Serializable> = Box::new(TagObject::new(repo.clone(), &payload));
  object::write(&new_tag, false)
}
