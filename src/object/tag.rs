use super::GitObject;

pub struct GitTag {
    pub data: Vec<u8>,
}

impl GitObject for GitTag {
    fn serialize(&self) -> &[u8] { &self.data[..] }
    fn object_type(&self) -> &str { "tag" }

    fn size(&self) -> usize {
        unimplemented!()
    }

    fn content(&self) -> String {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => s,
            Err(_e) => "".to_string(),
        }
    }

    fn new(data: Vec<u8>) -> Self {
        GitTag {
            data,
        }
    }
}
