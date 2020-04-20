use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub struct GitRepository {
    worktree: PathBuf,
    git_dir: PathBuf,
}

const INIT_DIRS: &[&str] = &[
    "objects",
    "refs/tags",
    "refs/heads",
];

/// A git repository
impl GitRepository {
    pub fn init(mut worktree: PathBuf) {
        worktree = worktree.canonicalize().unwrap();
        if !worktree.is_dir() {
            panic!("Invalid Path: {}\n", worktree.display())
        }

        let git_dir = worktree.join(".git");

        let repo = GitRepository {
            worktree,
            git_dir,
        };

        // Only do initialization where there is no .git directory
        if repo.git_dir.is_dir() {
            return;
        }

        // Do initialization.
        for dir in INIT_DIRS.iter().map(PathBuf::from) {
            create_dir_all(repo.repo_file(dir));
        }

        fs::write(repo.repo_file("HEAD"), "ref: refs/heads/master\n");
        fs::write(repo.repo_file("description"), "Unnamed repository; edit this file 'description' to name the repository.\n");
        // TODO configuration file, maybe with rust-ini(https://github.com/zonyitoo/rust-ini)
    }

    fn repo_file<P: AsRef<Path>>(&self, name: P) -> PathBuf {
        self.git_dir.join(name)
    }
}

