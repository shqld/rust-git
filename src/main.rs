use std::env;
use std::path::Path;

mod objects;
mod utils;

use objects::blob;
use objects::object;
use objects::object::Object;

use utils::digest;
use utils::fs;
use utils::zlib;

// Calculate digest first, and then compress it.
fn rit_add(path_str: &String) {
    let file_content = fs::cat(&Path::new(path_str)).unwrap();

    let blob_string = blob::Blob::create_store(&file_content);
    let hashed = digest::digest(&blob_string);
    let compressed = zlib::compress(&blob_string);

    let path = utils::get_file_path(&hashed);

    fs::write_file(&Path::new(&path), &compressed);
}

fn rit_cat_file_p(hash: &String) {
    let path = utils::get_file_path(&hash);
    let raw_object = fs::cat_binary(&Path::new(&path)).unwrap();

    let decoded = zlib::decompress(raw_object);
    let object = object::get_object(&decoded);
    println!("{}", object.get_content());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];

    match cmd.as_ref() {
        "cat-file" => rit_cat_file_p(&args.get(2).unwrap()),
        "add" => rit_add(&args.get(2).unwrap()),
        _ => println!("None"),
    }
}
