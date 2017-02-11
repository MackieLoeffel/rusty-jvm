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
    let classloader = ClassLoader::new("./assets");

    let mut vm = VM::new(classloader);
    match vm.start("Jump", &["arg1", "arg2"]) {
        Ok(..) => {}
        Err(ref err) => println!("Error running: {}", err),
    };
}
