use objects::file_mode::FileMode;
use objects::object::Object;
use utils;

pub struct Blob {
    header: String,
    content: String,
}

impl Blob {
    // fn new meta: String, content: String) -> Blob {
    //     Blob { meta, content }
    // }

    pub fn new(s: &String) -> Blob {
        let null_offset = s.find('\0').unwrap();

        Blob {
            header: s[..null_offset].to_string(),
            content: s[null_offset + 1..].to_string(),
        }
    }

    // "blob<space><size><null><content>"
    pub fn create_store(content: &String) -> String {
        format!(
            "blob {size}\0{content}",
            size = utils::get_bytes_size(content),
            content = content
        )
    }
}

impl Object for Blob {
    fn get_mode(&self) -> FileMode {
        FileMode::BLOB
    }
    fn get_header(&self) -> &String {
        &self.header
    }
    fn get_content(&self) -> &String {
        &self.content
    }
}
