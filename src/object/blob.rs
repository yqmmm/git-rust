use super::GitObject;

pub struct GitBlob {
    pub data: Vec<u8>,
}

impl GitObject for GitBlob {
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

    fn new(data: Vec<u8>) -> Self {
        GitBlob {
            data,
        }
    }
}
