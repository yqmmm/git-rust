use std::io::{BufRead, BufReader, Read};
use itertools::Itertools;

use super::GitObject;

pub struct GitTreeEntry {
    mode: u32,
    filename: String,
    sha: String,
}

impl std::fmt::Display for GitTreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:06} {} {}", self.mode, self.sha, self.filename)
    }
}

pub struct GitTree {
    pub data: Vec<u8>,
    items: Vec<GitTreeEntry>,
}

impl GitObject for GitTree {
    fn serialize(&self) -> &[u8] { &self.data[..] }

    fn object_type(&self) -> &str { "tree" }

    fn content(&self) -> String {
        self.items.iter()
            .join("\n")
    }
}

impl GitTree {
    pub fn new(data: Vec<u8>) -> Self {
        let mut reader = BufReader::new(&data[..]);
        let mut items = Vec::new();

        loop {
            let mut mode = Vec::new();
            reader.read_until(0x20, &mut mode);
            if mode.is_empty() {
                break;
            }
            mode.pop();
            let m: u32 = std::str::from_utf8(&mode).unwrap().parse().unwrap();

            let mut filename = Vec::new();
            reader.read_until(0, &mut filename);
            filename.pop();

            // SHA is 20 bytes in binary format
            let mut sha = [0; 20];
            reader.read_exact(&mut sha);
            let sha_str = hex::encode(sha);

            match String::from_utf8(filename) {
                Ok(f) => items.push(GitTreeEntry {
                    mode: m,
                    filename: f,
                    sha: sha_str,
                }),
                Err(err) => {
                    println!("{}", err);
                }
            };
        }

        return GitTree {
            data,
            items,
        };
    }
}
