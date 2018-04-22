extern crate sha1;
use self::sha1::{Digest, Sha1};

pub fn digest(string: &String) -> String {
  let hashed = Sha1::digest_str(string);
  format!("{:x}", hashed)
}

// pub fn digest_bytes(bytes: &Vec<u8>) -> String {
//     let hashed = Sha1::digest(bytes);
//     format!("{:x}", hashed)
// }
