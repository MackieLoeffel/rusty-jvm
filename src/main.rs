#![cfg_attr(feature = "strict", deny(warnings))]

extern crate classfile_parser;
#[macro_use]
extern crate nom;

mod class_loader;
mod class;
mod parsed_class;
mod instruction;
mod errors;
mod vm;
mod descriptor;

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
