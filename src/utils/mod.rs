pub mod digest;
pub mod fs;
pub mod zlib;

pub fn get_bytes_size(string: &String) -> usize {
  string.bytes().len()
}

pub fn get_file_path(hashed: &String) -> String {
  format!("./.rit/objects/{}/{}", &hashed[..2], &hashed[2..])
}
