use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

pub use blob::GitBlob;
pub use commit::GitCommit;
pub use tag::GitTag;
pub use tree::GitTree;

use crate::repo::GitRepository;

pub mod blob;
pub mod tree;
pub mod commit;
pub mod tag;

pub trait GitObject {
    fn new(data: Vec<u8>, repo: &GitRepository) -> Self where Self: Sized;
    fn serialize(&self) -> &[u8];
    fn object_type(&self) -> &str;
    fn size(&self) -> usize {
        return self.serialize().len();
    }
    fn content(&self) -> String;
}

pub fn hash_object(object: &impl GitObject, write: bool) -> String {
    let data = object.serialize();

    let mut hasher = Sha1::new();
    hasher.input(object.object_type());
    hasher.input(" ");
    hasher.input(data.len().to_string());
    hasher.input("\0");
    hasher.input(data);

    let hash = hex::encode(&hasher.result()[..]);

    if write {
        let repo = GitRepository::default();
        let filename = repo.repo_files(&["objects", &hash.as_str()[..2], &hash.as_str()[2..]]);
        let dir = filename.parent().unwrap();
        if !dir.is_dir() {
            create_dir_all(dir).unwrap();
        }
        let file = File::create(filename).unwrap();
        let mut zlib_writer = ZlibEncoder::new(file, flate2::Compression::default());
        // TODO remove this redundant code
        zlib_writer.write(object.object_type().as_bytes()).unwrap();
        zlib_writer.write(b" ").unwrap();
        zlib_writer.write(data.len().to_string().as_bytes()).unwrap();
        zlib_writer.write(b"\0").unwrap();
        zlib_writer.write(data).unwrap();
    }

    hash
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::object::GitBlob;

    use super::*;

    #[test]
    fn test_write_object() {
        let obj = GitBlob {
            data: "hello world\x0a".as_bytes().to_vec(),
        };
        assert_eq!(hash_object(&obj, false), "3b18e512dba79e4c8300dd08aeb37f8e728b8dad");
    }
}
