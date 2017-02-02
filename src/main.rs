#![cfg_attr(feature = "strict", deny(warnings))]

extern crate classfile_parser;

mod class_loader;

use class_loader::ClassLoader;

fn main() {
    let _classloader = ClassLoader::new("./assets");

    let parsed_class = classfile_parser::parse_class("./assets/SimpleClass").unwrap();
    println!("Version: {}.{}",
             parsed_class.minor_version,
             parsed_class.major_version);
}
