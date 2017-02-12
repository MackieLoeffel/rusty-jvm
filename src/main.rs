#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]
#![cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]
#![cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
#![cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
#![cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(or_fun_call))]
#![cfg_attr(feature = "cargo-clippy", allow(single_match))]
#![cfg_attr(feature = "cargo-clippy", allow(absurd_extreme_comparisons))]

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

pub const CLASSFILE_DIR: &'static str = "./java";

fn main() {
    let classloader = ClassLoader::new(CLASSFILE_DIR);

    let mut vm = VM::new(classloader);
    match vm.start("Jump", &["arg1", "arg2"]) {
        Ok(..) => {}
        Err(ref err) => println!("Error running: {}", err),
    };
}
