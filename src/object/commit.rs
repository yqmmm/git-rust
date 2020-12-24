use super::GitObject;

pub struct GitCommit {
    pub data: Vec<u8>,
}

impl GitObject for GitCommit {
    fn serialize(&self) -> &[u8] { &self.data[..] }
    fn object_type(&self) -> &str { "commit" }

    fn size(&self) -> usize {
        unimplemented!()
    }

    fn content(&self) -> String {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => s,
            Err(err) => {
                println!("{}", err);
                "".to_string()
            },
        }
    }
}
