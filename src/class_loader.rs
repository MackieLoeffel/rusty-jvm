use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use classfile_parser::class_parser_option;
use class::Class;
use errors::ClassLoadingError;

// see https://docs.oracle.com/javase/specs/jvms/se6/html/ConstantPool.doc.html

// we implement the second edition of the jvm, so we have to support version 45.0 to 46.0
// ttps://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html#75883
const MIN_MAJOR_VERSION: u16 = 45;
const MIN_MINOR_VERSION: u16 = 0;
const MAX_MAJOR_VERSION: u16 = 46;
const MAX_MINOR_VERSION: u16 = 0;

pub struct ClassLoader {
    load_dir: PathBuf,
    loaded_classes: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new(load_dir: &str) -> ClassLoader {
        ClassLoader {
            load_dir: load_dir.into(),
            loaded_classes: HashMap::new(),
        }
    }

    pub fn load_class(&mut self, name: &str) -> Result<&Class, ClassLoadingError> {
        // must check twice to make the borrow-checker happy
        // TODO change, when non-lexical-lifetimes arrive
        if self.loaded_classes.contains_key(name) {
            return match self.loaded_classes.get(name) {
                Some(c) => Ok(c),
                None => unreachable!(),
            };
        }
        self.load_file(name)
    }

    fn load_file(&mut self, name: &str) -> Result<&Class, ClassLoadingError> {
        // println!("Loading class: {}", name);
        let classfilename = format!("{}.class", name.split('/').last().unwrap_or(name));
        let mut file = match File::open(self.load_dir.join(classfilename)) {
            Ok(file) => file,
            Err(err) => return Err(ClassLoadingError::NoClassDefFound(Some(err))),
        };
        let mut bytes = Vec::new();
        match file.read_to_end(&mut bytes) {
            Ok(..) => {}
            Err(err) => return Err(ClassLoadingError::NoClassDefFound(Some(err))),
        };

        let classfile = match class_parser_option(&bytes) {
            Some(classfile) => classfile,
            None => return Err(ClassLoadingError::ClassFormatError("Can't parse class".to_owned())),
        };

        if classfile.major_version < MIN_MAJOR_VERSION ||
           (classfile.major_version == MIN_MAJOR_VERSION && classfile.minor_version < MIN_MINOR_VERSION) ||
           classfile.major_version > MAX_MAJOR_VERSION ||
           (classfile.major_version == MAX_MAJOR_VERSION && classfile.minor_version > MAX_MINOR_VERSION) {
            return Err(ClassLoadingError::UnsupportedClassVersion);
        }

        let class = match Class::from_class_file(&classfile) {
            Ok(c) => c,
            Err(s) => return Err(ClassLoadingError::ClassFormatError(s)),
        };

        let class_name = class.name().to_owned();
        assert!(self.loaded_classes.insert(class_name.clone(), class).is_none());
        if class_name != name {
            return Err(ClassLoadingError::ClassFormatError(format!("Expected class {}, but found {}",
                                                                   name,
                                                                   class_name)));
        }

        Ok(&self.loaded_classes[&class_name])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> ClassLoader { ClassLoader::new(super::super::CLASSFILE_DIR) }

    #[test]
    fn not_existing_class() {
        let mut classloader = setup();
        assert!(match classloader.load_class("NotExistingClass").err() {
            Some(ClassLoadingError::NoClassDefFound(..)) => true,
            _ => false,
        });
    }

    #[test]
    fn unsupported_class_version() {
        let mut classloader = setup();
        assert!(match classloader.load_class("UnsupportedClassVersion").err() {
            Some(ClassLoadingError::UnsupportedClassVersion) => true,
            _ => false,
        });
    }

    #[test]
    fn malformed_class() {
        let mut classloader = setup();
        assert!(match classloader.load_class("malformed").err() {
            Some(ClassLoadingError::ClassFormatError(..)) => true,
            _ => false,
        });
    }

    #[test]
    fn good_class() {
        let mut classloader = setup();
        let class = classloader.load_class("com/mackie/rustyjvm/TestClass").unwrap();
        assert_eq!(class.name(), "com/mackie/rustyjvm/TestClass");
    }
}
