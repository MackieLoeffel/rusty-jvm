#![cfg_attr(feature = "strict", deny(warnings))]

extern crate classfile_parser;

mod class_loader;
mod class;
mod instruction;

use class_loader::ClassLoader;

fn main() {
    let mut classloader = ClassLoader::new("./assets");

    let parsed_class = classloader.load_class("HelloWorld").unwrap();
    println!("Class: {:?}", parsed_class);
}
