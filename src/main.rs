#![cfg_attr(feature = "strict", deny(warnings))]

extern crate classfile_parser;

mod class_loader;
mod class;

use class_loader::ClassLoader;

fn main() {
    let mut classloader = ClassLoader::new("./assets");

    let parsed_class = classloader.load_class("SimpleClass").unwrap();
    println!("Name: {}", parsed_class.name());
}
