use std::io::{Cursor, Read};

use super::GitObject;

pub struct GitBlob {
    pub size: u32,
    pub data: Vec<u8>,
}

impl GitObject for GitBlob {
    fn serialize(&self) -> &[u8] { &self.data[..] }
    fn object_type(&self) -> &str { "blob" }
    fn size(&self) -> u32 { self.size }
}


