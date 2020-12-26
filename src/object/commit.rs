use std::collections::HashMap;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom};
use std::rc::Rc;

use crate::repo::GitRepository;

use super::GitObject;

pub struct GitCommit {
    pub data: String,
    pub tree: String,
    pub parents: Vec<String>,
    pub author: String,
    pub committer: String,

    parent: Option<Rc<Box<dyn GitObject>>>, // TODO: is Box in Rc the right thing to do?
}

impl GitObject for GitCommit {
    fn new(data: Vec<u8>, repo: &GitRepository) -> Self {
        let data_str = String::from_utf8(data).unwrap();
        let headers = read_commit_header(&data_str);

        let mut parents = Vec::new();
        let mut tree = String::new();
        let mut author = String::new();
        let mut committer = String::new();

        headers.into_iter().for_each(|(k, v)| {
            match &k[..] {
                "tree" => tree = v,
                "parent" => parents.push(v),
                "author" => author = v,
                "committer" => committer = v,
                _ => (),
            }
        });

        GitCommit {
            data: data_str,
            tree,
            parents,
            author,
            committer,
            parent: None,
        }
    }

    fn serialize(&self) -> &[u8] { self.data.as_bytes() }

    fn object_type(&self) -> &str { "commit" }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn content(&self) -> String {
        format!("commit:{}\nAuthor:{}\n", self.tree, self.committer)
    }
}

// Commit object starts with some key-value pairs
// Read it hear
// TODO: Error handling
fn read_commit_header(data: &str) -> Vec<(String, String)> {
    let cursor = Cursor::new(data);

    let mut kv = Vec::new();
    cursor.lines()
        .filter_map(|result| result.ok())
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let mut split = line.splitn(2, " ");
            kv.push((split.next().unwrap().to_string(), split.next().unwrap().to_string()));
        });
    kv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_into_kv() {
        let data = "tree parent1\ntree parent2\ncommitter me\n\nI'm the commit message";

        let kv = read_commit_header(data);

        assert_eq!(kv.len(), 3);
    }
}
