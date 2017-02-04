use nom::IResult;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use classfile_parser::{ClassFile, class_parser};

// see https://docs.oracle.com/javase/specs/jvms/se6/html/ConstantPool.doc.html

// we implement the second edition of the jvm, so we have to support version 45.0 to 46.0
// ttps://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html#75883
const MIN_MAJOR_VERSION: u16 = 45;
const MIN_MINOR_VERSION: u16 = 0;
const MAX_MAJOR_VERSION: u16 = 46;
const MAX_MINOR_VERSION: u16 = 0;

pub struct ClassLoader {
    load_dir: PathBuf
}

impl ClassLoader {
    pub fn new(load_dir: &str) -> ClassLoader {
        ClassLoader { load_dir: load_dir.into() }
    }

    pub fn load_class(&mut self, name: &str) -> Result<ClassFile, ClassLoadingError> {
        let classfilename = format!("{}.class", name);
        let mut file = match File::open(self.load_dir.join(classfilename)) {
            Ok(file) => file,
            Err(..) => return Err(ClassLoadingError::NoClassDefFound)
        };
        let mut bytes = Vec::new();
        match file.read_to_end(&mut bytes) {
            Ok(..) => {},
            Err(..) => return Err(ClassLoadingError::NoClassDefFound)
        };

        let classfile = match class_parser(&bytes) {
            IResult::Done(_, classfile) => classfile,
            _ => return Err(ClassLoadingError::ClassFormatError),
        };

        if classfile.major_version < MIN_MAJOR_VERSION || (classfile.major_version == MIN_MAJOR_VERSION && classfile.minor_version < MIN_MINOR_VERSION) || classfile.major_version > MAX_MAJOR_VERSION || (classfile.major_version == MAX_MAJOR_VERSION && classfile.minor_version > MAX_MINOR_VERSION) {
            return Err(ClassLoadingError::UnsupportedClassVersion);
        }

        Ok(classfile)
    }
}

// TODO implement Error and use ?-Operator above
#[derive(Debug)]
pub enum ClassLoadingError {
    NoClassDefFound,
    ClassFormatError,
    UnsupportedClassVersion,
    #[allow(dead_code)]
    IncompatibleClassChange,
    #[allow(dead_code)]
    ClassCircularity,
}
