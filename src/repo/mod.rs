extern crate ini;

use ini::Ini as ConfigParser;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

/// A git repository.
///
/// In git, a repository is made up of a `working tree` and a `git directory`.
/// The working tree is the directory where the files that are tracked by git
/// are stored. The git directory is where git keeps its metadata. The git
/// directory is usually located in the `.git` directory in the working tree.
pub struct Repo {
    /// The path to the git directory.
    pub git_dir: PathBuf,

    /// The path to the working tree.
    pub work_tree: PathBuf,

    /// Parses config (.ini) file in `.git/config`
    pub config: Option<ConfigParser>,
}

impl Repo {
    /// Initializes a new git repository.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the working tree.
    /// * `force` - If true, the repository will be created even from an invalid
    /// filesystem location.
    pub fn init(path: &PathBuf, force: bool) -> Result<Repo, String> {
        // If we are not forcing creation, the path must exist.
        if !force && !path.exists() {
            return Err(format!("{} does not exist.", path.display()));
        }

        // Try to read in the config file inside the `.git` directory.
        let mut config: Option<ConfigParser> = None;
        let git_dir = path.join(".git");
        match repo_file(&git_dir, &["config"], false) {
            Some(config_file) => {
                if config_file.exists() {
                    match ConfigParser::load_from_file(config_file) {
                        Ok(conf) => config = Some(conf),
                        Err(error) => return Err(error.to_string()),
                    };
                } else if !force {
                    return Err(format!("Configuration file is missing."));
                }
            }
            None => {
                if !force {
                    return Err(format!("{} is not a git repository.", path.display()));
                }
            }
        }

        // If we are not forcing creation, the `repositoryformatversion`
        // must be equal to 0.
        if !force {
            if let Some(ref parser) = config {
                if let Some(core) = parser.section(Some("core")) {
                    if let Some(version) = core.get("repositoryformatversion") {
                        if version != "0" {
                            return Err(format!(
                                "Unsupported repository format version: {}",
                                version
                            ));
                        }
                    }
                };
            } else {
                return Err("repo config parser invalid".to_string());
            }
        }
        Ok(Self {
            git_dir,
            work_tree: path.to_path_buf(),
            config,
        })
    }

    /// Create a new repository.
    ///
    /// This will create a new repository at the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the repository.
    pub fn new(path: &PathBuf) -> Result<Repo, String> {
        match Repo::init(path, true) {
            Ok(repo) => {
                // First, make sure the path either doesn't exist or is an empty directory.
                if repo.work_tree.exists() {
                    let dir = repo.work_tree.display();
                    if !repo.work_tree.is_dir() {
                        return Err(format!("{} is not a directory", dir));
                    }
                    if repo.work_tree.read_dir().unwrap().count() != 0 {
                        return Err(format!("{} is not empty", dir));
                    }
                } else {
                    if !create_dir_all(&repo.work_tree).is_ok() {
                        panic!("failed to create {}", repo.work_tree.display());
                    }
                }

                // Verify that the repository has been successfully created.
                repo_dir(&repo.git_dir, &["branches"], true);
                repo_dir(&repo.git_dir, &["objects"], true);
                repo_dir(&repo.git_dir, &["refs", "tags"], true);
                repo_dir(&repo.git_dir, &["refs", "heads"], true);

                // Write the default `.git/description` file.
                let data = "Edit this file to name this repository.\n";
                let path = repo_file(&repo.git_dir, &["description"], true);
                Repo::write_to_file(data, &path.unwrap());

                // Write the default `.git/description` file.
                let data = "ref: refs/heads/master\n";
                let path = repo_file(&repo.git_dir, &["HEAD"], true);
                Repo::write_to_file(data, &path.unwrap());

                // Write the default `.git/config` file.
                let config = Repo::repo_default_config();
                let path = repo_file(&repo.git_dir, &["config"], true);
                config.write_to_file(path.unwrap()).unwrap();

                return Ok(repo);
            }
            Err(error) => return Err(error),
        }
    }

    /// Walk up the directory tree to find the root of the repository (`.git`).
    pub fn find_repo(path: &PathBuf, required: bool) -> Result<Option<Repo>, String> {
        // Shadow the path parameter with its absolute path.
        let path = path.canonicalize().unwrap();

        // If the path has a `.git` directory, we are done.
        if path.join(".git").is_dir() {
            match Repo::new(&path) {
                Ok(repo) => return Ok(Some(repo)),
                Err(error) => return Err(error),
            }
        }

        // Otherwise, we need to walk up the directory tree.
        match path.parent() {
            Some(parent) => return Repo::find_repo(&parent.to_path_buf(), required),
            None => {
                if required {
                    return Err("Could not find a git directory".to_string());
                } else {
                    return Ok(None);
                }
            }
        }
    }

    /// Write the given data to the given path. Panic on error.
    fn write_to_file(data: &str, path: &PathBuf) {
        match File::create(path) {
            Ok(mut f) => match f.write_all(data.as_bytes()) {
                Ok(_) => (),
                Err(_) => panic!("unable to write data"),
            },
            Err(_) => panic!("unable to create file"),
        }
    }

    /// Builds up a default configuration for a new repository.
    fn repo_default_config() -> ConfigParser {
        let mut conf = ConfigParser::new();
        conf.with_section(Some("core"))
            .set("repositoryformatversion", "0") // use the initial gitdir format
            .set("filemode", "false") // don't track file mode changes in worktree
            .set("bare", "false"); // indicates this repo has a worktree
        conf
    }
}

/// Returns a new PathBuf with the given path appended to the given pathbuf.
fn repo_path(git_dir: &PathBuf, paths: &[&str]) -> PathBuf {
    let mut new_path = git_dir.clone();
    new_path.extend(paths.into_iter());
    return new_path;
}

/// Computes path under repo's git directory, and creates the directory if
/// it does not exist.
///
/// # Examples
/// ```
/// repo_file(r, "refs", "remotes", "origin", "HEAD")
/// ```
/// will create `.git/refs/remotes/origin` if it does not exist.
pub fn repo_file(root: &PathBuf, path: &[&str], mkdir: bool) -> Option<PathBuf> {
    match repo_dir(root, &path[..path.len() - 1], mkdir) {
        Some(file_path) => {
            if file_path.exists() {
                Some(repo_path(root, path))
            } else {
                panic!("{} does not exist", file_path.display());
            }
        }
        None => None,
    }
}

/// Computes path under repo's git directory, and creates the directory if
/// it does not exist.
fn repo_dir(root: &PathBuf, path: &[&str], mkdir: bool) -> Option<PathBuf> {
    // If the directory does not exist, create it.
    let path = repo_path(root, path);
    if path.exists() {
        if path.is_dir() {
            return Some(path);
        } else {
            panic!("{} is not a directory", path.display());
        }
    }

    // The path does not exist; create it if we are allowed to.
    if mkdir {
        create_dir_all(&path).unwrap();
        return Some(path);
    } else {
        return None;
    }
}
