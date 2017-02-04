#![cfg_attr(feature = "strict", deny(warnings))]

extern crate classfile_parser;
extern crate nom;

mod class_loader;

use class_loader::ClassLoader;

fn main() {
    let mut classloader = ClassLoader::new("./assets");

    let parsed_class = classloader.load_class("SimpleClass").unwrap();
    println!("Version: {}.{}",
             parsed_class.major_version,
             parsed_class.minor_version);
}
