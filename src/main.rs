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
mod object;

use class_loader::ClassLoader;
use vm::VM;
use std::env;
use std::process::exit;
use std::io::{stderr, Write};

pub const CLASSFILE_DIR: &'static str = "./java";

fn main() {
    let dest = match env::args().nth(1) {
        Some(s) => s,
        None => {
            writeln!(&mut stderr(),
                     "Usage: {} <classname> <args>",
                     env::args().nth(0).unwrap())
                .expect("stderr writing failed");
            exit(1);
        }
    };

    let classloader = ClassLoader::new(CLASSFILE_DIR);

    let mut vm = VM::new(classloader);
    match vm.start(&dest, &[]) {
        Ok(..) => {}
        Err(ref err) => {
            writeln!(&mut stderr(), "Error running: {}", err).expect("stderr writing failed");
            exit(1);
        }
    };
}
