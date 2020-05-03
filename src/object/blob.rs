use super::GitObject;

pub struct GitBlob {
    pub data: Vec<u8>,
}

impl GitObject for GitBlob {
    fn serialize(&self) -> &[u8] { &self.data[..] }
    fn object_type(&self) -> &str { "blob" }
}
