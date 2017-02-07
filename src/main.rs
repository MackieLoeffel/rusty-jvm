#![cfg_attr(feature = "strict", deny(warnings))]

extern crate classfile_parser;

mod class_loader;
mod class;
mod instruction;
mod errors;
mod vm;

use class_loader::ClassLoader;
use vm::VM;

fn main() {
    let mut classloader = ClassLoader::new("./assets");

    {
        let parsed_class = classloader.load_class("HelloWorld").unwrap();
        println!("Class: {:?}", parsed_class);
    }

    let mut vm = VM::new(classloader);
    match vm.start("HelloWorld", &["arg1", "arg2"]) {
        Ok(..) => println!("Finished!"),
        Err(ref err) => println!("Error running: {}", err),
    };
}
