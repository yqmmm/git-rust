use std::fs;
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use flate2::bufread::ZlibDecoder;

use crate::object::blob::GitBlob;

use super::object;

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
    pub fn default() -> GitRepository {
        GitRepository {
            worktree: PathBuf::from("."),
            git_dir: PathBuf::from(".git"),
        }
    }

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
            create_dir_all(repo.repo_file(dir)).unwrap();
        }

        fs::write(repo.repo_file("HEAD"), "ref: refs/heads/master\n").unwrap();
        fs::write(repo.repo_file("description"), "Unnamed repository; edit this file 'description' to name the repository.\n").unwrap();
        // TODO configuration file, maybe with rust-ini(https://github.com/zonyitoo/rust-ini)
    }

    pub fn read_object(sha: &str) {
        if sha.len() != 40 || !sha.is_ascii() {
            panic!("Invalid SHA-1 Value")
        }

        let repo = GitRepository::default();
        let mut dirs = vec!["objects"];
        dirs.push(&sha[..2]);
        dirs.push(&sha[2..]);
        let filename = repo.repo_files(dirs);

        let file = File::open(filename).unwrap();
        let mut file_reader = BufReader::new(file);

        let object = object::new(&mut file_reader);
        println!("Type: {}", object.object_type());
        println!("Size: {}", object.size());
    }

    fn repo_file<P: AsRef<Path>>(&self, name: P) -> PathBuf {
        self.git_dir.join(name)
    }

    fn repo_files<P, PI>(&self, name: PI) -> PathBuf
        where PI: IntoIterator<Item=P>,
              P: AsRef<Path>
    {
        let mut file = self.git_dir.clone();
        name.into_iter().for_each(|p| file.push(p));
        file
    }
}

#[cfg(test)]
mod test {
    use std::iter;
    use std::path::PathBuf;

    use crate::repo::GitRepository;

    #[test]
    fn repo_files() {
        let files = &["objects/", "3b", "18e512dba79e4c8300dd08aeb37f8e728b8dad"];
        let repo = GitRepository {
            worktree: PathBuf::from("."),
            git_dir: PathBuf::from(".git"),
        };

        assert_eq!(repo.repo_files(files),
                   PathBuf::from(".git/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad"))
    }
}

