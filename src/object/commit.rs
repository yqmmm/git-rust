use std::io::{BufReader, Read, BufRead, Cursor, Seek, SeekFrom};

use super::GitObject;

pub struct GitCommit {
    pub data: Vec<u8>,
    tree: String,
    parent: String,
}

impl GitObject for GitCommit {
    fn serialize(&self) -> &[u8] { &self.data[..] }
    fn object_type(&self) -> &str { "commit" }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn content(&self) -> String {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => s,
            Err(err) => {
                println!("{}", err);
                "".to_string()
            }
        }
    }

    fn new(data: Vec<u8>) -> Self {
        let mut cursor = Cursor::new(data);

        let mut tree_vec = Vec::new();
        cursor.seek(SeekFrom::Start(5));
        cursor.read_until(b'\n', &mut tree_vec);
        tree_vec.pop();
        let tree = String::from_utf8(tree_vec).unwrap();
        println!("{}", tree);

        let mut parent_vec = Vec::new();
        cursor.seek(SeekFrom::Current(7));
        cursor.read_until(b'\n', &mut parent_vec);
        parent_vec.pop();
        let parent = String::from_utf8(parent_vec).unwrap();
        println!("{}", parent);

        GitCommit {
            data: cursor.into_inner(),
            tree,
            parent,
        }
    }
}
