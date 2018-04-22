use objects::file_mode::FileMode;
use objects::object::Object;
use utils;

pub struct Tree {
    header: String,
    content: String,
}

impl Tree {
    // fn new meta: String, content: String) -> Tree {
    //     Tree { meta, content }
    // }

    pub fn new(s: &String) -> Tree {
        let null_offset = s.find('\0').unwrap();

        Tree {
            header: s[..null_offset].to_string(),
            content: s[null_offset + 1..].to_string(),
        }
    }

    // "Tree<space><size><null><content>"
    pub fn create_store(items: Vec<String>) -> String {
        // let content =

        format!(
            "tree {size}\0{content}",
            size = utils::get_bytes_size(content),
            content = content
        )
    }
}

impl Object for Tree {
    fn get_mode(&self) -> FileMode {
        FileMode::Tree
    }
    fn get_header(&self) -> &String {
        &self.header
    }
    fn get_content(&self) -> &String {
        &self.content
    }
}
