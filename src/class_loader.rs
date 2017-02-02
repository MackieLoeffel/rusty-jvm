// use classfile_parser::{ClassFile, parse_class};

// see https://docs.oracle.com/javase/specs/jvms/se6/html/ConstantPool.doc.html

pub struct ClassLoader {
    // TODO: convert to Path when classfile_parser is changed
    #[allow(dead_code)]
    load_dir: String,
}

impl ClassLoader {
    pub fn new(load_dir: &str) -> ClassLoader {
        ClassLoader { load_dir: load_dir.into() }
    }

    // pub fn load_class(name: &str) -> Result<>
}

#[allow(dead_code)]
pub enum ClassLoadingError {
    NoClassDefFound,
    ClassFormatError,
    UnsupportedClassVersion,
    IncompatibleClassChange,
    ClassCircularity,
}
