use std::io::{BufRead, BufReader, Read, Write, BufWriter};

use flate2::read::{ZlibDecoder, ZlibEncoder};
use sha1::{Digest, Sha1};
use sha1::digest::generic_array::GenericArray;

pub mod blob;
pub mod tree;

pub trait GitObject {
    fn serialize(&self) -> &[u8];
    fn object_type(&self) -> &str;
    fn size(&self) -> u32;
}

pub fn new<R: Read>(input: &mut R) -> Box<dyn GitObject> {
    let mut zlib_reader = BufReader::new(ZlibDecoder::new(input));

    let mut type_vec = Vec::new();
    let mut size_vec = Vec::new();
    let mut content_vec = Vec::new();

    zlib_reader.read_until(0x20, &mut type_vec).unwrap();
    type_vec.pop();
    zlib_reader.read_until(0, &mut size_vec).unwrap();
    size_vec.pop();
    zlib_reader.read_to_end(&mut content_vec).unwrap();

    let size: u32 = String::from_utf8(size_vec).unwrap().parse().unwrap();

    Box::new(match &type_vec[..] {
        b"blob" => blob::GitBlob {
            size,
            data: content_vec,
        },
        _ => panic!()
    })
}

pub fn hash_object<W: Write>(object: impl GitObject, write: Option<&mut W>) -> String {
    let data = object.serialize();

    let mut hasher = Sha1::new();
    hasher.input(object.object_type());
    hasher.input(" ");
    hasher.input(data.len().to_string());
    hasher.input("\0");
    hasher.input(data);

    let result = hex::encode(&hasher.result()[..]);

    if write.is_some() {
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::object::blob::GitBlob;

    use super::*;

    #[test]
    fn test_write_object() {
        let obj = GitBlob {
            size: 12,
            data: "hello world\x0a".as_bytes().to_vec(),
        };
        assert_eq!(hash_object(obj, false), "3b18e512dba79e4c8300dd08aeb37f8e728b8dad");
    }
}
