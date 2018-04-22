pub enum FileMode {
  NEW,
  TREE,
  BLOB,
  // BLOB_EXECUTABLE,
  // LINK,
  COMMIT,
}

impl FileMode {
  pub fn to_code(&self) -> String {
    use self::FileMode::*;

    let code = match self {
      NEW => 0000000,
      TREE => 0040000,
      BLOB => 0100644,
      // BLOB_EXECUTABLE => 0100755,
      // LINK => 0120000,
      COMMIT => 0160000,
    };
    code.to_string()
  }

  pub fn from_string(s: &String) -> FileMode {
    let space_offset = s.find(' ').unwrap();

    use self::FileMode::*;
    match &s[..space_offset] {
      // "new"
      "tree" => TREE,
      "blob" => BLOB,
      // "bllob_executable"
      // "link"
      "commit" => COMMIT,
      _ => NEW,
    }
  }
}
