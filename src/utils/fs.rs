use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use std::io::Read;
use std::io::Write;

pub fn cat(path: &Path) -> io::Result<String> {
  let mut f = try!(File::open(path));
  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

pub fn cat_binary(path: &Path) -> io::Result<Vec<u8>> {
  let mut f = try!(File::open(path));
  let mut s = Vec::new();

  match f.read_to_end(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

pub fn write_file(path: &Path, content: &Vec<u8>) -> io::Result<()> {
  let dir_path = path.parent().unwrap();
  fs::create_dir_all(dir_path)?;

  let mut f = File::create(&path).unwrap();
  f.write_all(content)
}

// fn write_file_string(s: &String, path: &Path) -> io::Result<()> {
//     let mut f = try!(File::create(path));
//     f.write_all(s.as_bytes())
// }
