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
    pub fn init(path: &PathBuf, force: Option<bool>) -> Result<Repo, String> {
        // Shadow the force parameter with a default value (false).
        let force: bool = force.unwrap_or(false);

        // If we are not forcing creation, the path must exist.
        if !force && !path.exists() {
            return Err(format!("Not a Git repository {}", path.display()));
        }

        // Try to read in the config file inside the `.git` directory.
        let mut config: Option<ConfigParser> = None;
        let git_dir = path.join(".git");
        let config_file: PathBuf = Repo::repo_file(&git_dir, &["config"], true);
        if config_file.exists() {
            match ConfigParser::load_from_file(config_file) {
                Ok(conf) => config = Some(conf),
                Err(error) => {
                    return Err(format!("{}", error));
                }
            };
        } else if !force {
            return Err("Configuration file is missing".to_string());
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

    /// Returns a new PathBuf with the given path appended to the given pathbuf.
    fn repo_path(git_dir: &PathBuf, paths: &[&str]) -> PathBuf {
        let mut result = git_dir.clone();
        for path in paths.into_iter() {
            result.push(path);
        }
        result
    }

    /// Computes path under repo's git directory, and creates the directory if
    /// it does not exist.
    ///
    /// # Examples
    /// ```
    /// repo_file(r, "refs", "remotes", "origin", "HEAD")
    /// ```
    /// will create `.git/refs/remotes/origin/HEAD` if it does not exist.
    fn repo_file(root: &PathBuf, path: &[&str], mkdir: bool) -> PathBuf {
        let file_path = Repo::repo_dir(root, path, mkdir);
        if file_path.exists() {
            Repo::repo_path(root, path)
        } else {
            panic!("{} does not exist", file_path.display());
        }
    }

    /// Computes path under repo's git directory, and creates the directory if
    /// it does not exist.
    fn repo_dir(root: &PathBuf, path: &[&str], mkdir: bool) -> PathBuf {
        // If the directory does not exist, create it.
        let path = Repo::repo_path(root, path);
        if path.exists() {
            if path.is_dir() {
                return path;
            } else {
                panic!("{} is not a directory", path.display());
            }
        }

        // The path does not exist; create it if we are allowed to.
        if mkdir {
            create_dir_all(&path).unwrap();
            return path;
        } else {
            panic!("{} does not exist", path.display());
        }
    }
}

/// Create a new repository.
///
/// This will create a new repository at the given path.
///
/// # Arguments
///
/// * `path` - The path to the repository.
pub fn create(path: &PathBuf) -> Repo {
    let repo: Repo = Repo::init(path, None).unwrap();

    // First, make sure the path either doesn't exist or is an empty directory.
    if repo.work_tree.exists() {
        if !repo.work_tree.is_dir() {
            panic!("{} is not a directory", repo.work_tree.display());
        }
        if repo.work_tree.read_dir().unwrap().count() != 0 {
            panic!("{} is not empty", repo.work_tree.display());
        }
    } else {
        if !create_dir_all(&repo.work_tree).is_ok() {
            panic!("failed to create {}", repo.work_tree.display());
        }
    }

    // Verify that the repository has been successfully created.
    Repo::repo_dir(&repo.git_dir, &["branches"], true);
    Repo::repo_dir(&repo.git_dir, &["objects"], true);
    Repo::repo_dir(&repo.git_dir, &["refs", "tags"], true);
    Repo::repo_dir(&repo.git_dir, &["refs", "heads"], true);

    // Write the default `.git/description` file.
    let data = "Edit this file to name this repository.\n";
    let path = Repo::repo_file(&repo.git_dir, &["description"], true);
    write_to_file(data, &path);

    // Write the default `.git/description` file.
    let data = "ref: refs/heads/master\n";
    let path = Repo::repo_file(&repo.git_dir, &["HEAD"], true);
    write_to_file(data, &path);

    // Write the default `.git/config` file.
    let config = repo_default_config();
    let path = Repo::repo_file(&repo.git_dir, &["config"], true);
    config.write_to_file(path).unwrap();

    return repo;
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
