use objects::blob::Blob;
use objects::file_mode::FileMode;

pub trait Object {
  fn get_mode(&self) -> FileMode;
  fn get_header(&self) -> &String;
  fn get_content(&self) -> &String;
}

pub fn get_object(s: &String) -> impl Object {
  use self::FileMode::*;
  match FileMode::from_string(s) {
    BLOB => Blob::new(s),
    _ => Blob::new(s), // DEBUG
  }
}
