use std::fs;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;

use crate::object::*;

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

    pub fn new<R: Read>(&self, input: &mut R) -> Option<Box<dyn GitObject>> {
        let mut zlib_reader = BufReader::new(ZlibDecoder::new(input));

        let mut type_vec = Vec::new();
        let mut size_vec = Vec::new();
        let mut content_vec = Vec::new();

        zlib_reader.read_until(0x20, &mut type_vec).unwrap();
        type_vec.pop();
        zlib_reader.read_until(0, &mut size_vec).unwrap();
        size_vec.pop();
        zlib_reader.read_to_end(&mut content_vec).unwrap();

        match &type_vec[..] {
            b"blob" => Some(Box::new(GitBlob {
                data: content_vec,
            })),
            b"commit" => Some(Box::new(GitCommit::new(content_vec, self))),
            b"tree" => Some(Box::new(GitTree::new(content_vec, self))),
            b"tag" => Some(Box::new(GitTag {
                data: content_vec,
            })),
            _ => {
                println!("{:?}", &type_vec[..]);
                None
            }
        }
    }

    pub fn read_object(&self, sha: &str) -> Option<Box<dyn GitObject>> {
        if sha.len() != 40 || !sha.is_ascii() {
            panic!("Invalid SHA-1 Value: {}", sha)
        }

        let dirs = vec!["objects", &sha[..2], &sha[2..]];
        let filename = self.repo_files(dirs);

        let file = File::open(filename).unwrap();
        let mut file_reader = BufReader::new(file);

        self.new(&mut file_reader)
    }

    pub fn ref_resolve(&self, ref_name: &str) -> String {
        let filename = self.repo_file(ref_name);
        let file = File::open(filename).unwrap();
        BufReader::new(file).lines().next().unwrap().unwrap()
    }

    fn repo_file<P: AsRef<Path>>(&self, name: P) -> PathBuf {
        self.git_dir.join(name)
    }

    pub fn repo_files<T>(&self, name: T) -> PathBuf
        where T: IntoIterator,
              T::Item: AsRef<Path>
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

