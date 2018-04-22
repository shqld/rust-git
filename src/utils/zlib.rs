extern crate flate2;

use self::flate2::read::ZlibDecoder;
use self::flate2::write::ZlibEncoder;
use self::flate2::Compression;

use std::io::Read;
use std::io::Write;

pub fn compress(s: &String) -> Vec<u8> {
  let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
  e.write_all(s.as_bytes());
  e.finish().unwrap()
}

pub fn decompress(bytes: Vec<u8>) -> String {
  let mut z = ZlibDecoder::new(&bytes[..]);
  let mut s = String::new();
  z.read_to_string(&mut s);
  s
}
