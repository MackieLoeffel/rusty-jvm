extern crate classfile_parser;

fn main() {
    let parsed_class = classfile_parser::parse_class("./assets/SimpleClass").unwrap();
    println!("Version: {}.{}", parsed_class.minor_version, parsed_class.major_version);
}
