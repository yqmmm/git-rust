use super::GitObject;
use crate::repo::GitRepository;

pub struct GitBlob {
    pub data: Vec<u8>,
}

impl GitObject for GitBlob {
    fn new(data: Vec<u8>, repo: &GitRepository) -> Self {
        GitBlob {
            data,
        }
    }
    fn serialize(&self) -> &[u8] { &self.data[..] }

    fn object_type(&self) -> &str { "blob" }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn content(&self) -> String {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => s,
            Err(_e) => "".to_string(),
        }
    }
}
