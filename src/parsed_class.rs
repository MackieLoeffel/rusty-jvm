use classfile_parser::*;
use classfile_parser::constant_info::*;

pub trait ParsedClass {
    fn constant(&self, index: u16) -> Result<&ConstantInfo, String>;
    fn constant_utf8(&self, index: u16) -> Result<&str, String>;
    fn constant_class(&self, index: u16) -> Result<&str, String>;
    fn constant_name_and_type(&self, index: u16) -> Result<(&str, &str), String>;
    fn constant_field_ref(&self, index: u16) -> Result<FieldRef, String>;
    fn constant_method_ref(&self, index: u16) -> Result<MethodRef, String>;
    fn constant_interface_method_ref(&self, index: u16) -> Result<MethodRef, String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldRef {
    name: String,
    class: String,
    descriptor: String,
}
impl FieldRef {
    #[allow(dead_code)]
    pub fn new(name: &str, class: &str, descriptor: &str) -> FieldRef {
        FieldRef {
            name: name.to_owned(),
            class: class.to_owned(),
            descriptor: descriptor.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodRef {
    name: String,
    class: String,
    descriptor: String,
}
impl MethodRef {
    #[allow(dead_code)]
    pub fn new(name: &str, class: &str, descriptor: &str) -> MethodRef {
        MethodRef {
            name: name.to_owned(),
            class: class.to_owned(),
            descriptor: descriptor.to_owned(),
        }
    }
}

impl ParsedClass for ClassFile {
    fn constant(&self, index: u16) -> Result<&ConstantInfo, String> {
        if index == 0 || index as usize > self.const_pool.len() {
            return Err("index out of bounds".to_owned());
        }
        return Ok(&self.const_pool[(index - 1) as usize]);
    }

    fn constant_utf8(&self, index: u16) -> Result<&str, String> {
        match *self.constant(index)? {
            ConstantInfo::Utf8(ref s) => Ok(&s.utf8_string),
            _ => Err("Not a utf8 constant".to_owned()),
        }
    }

    fn constant_class(&self, index: u16) -> Result<&str, String> {
        match *self.constant(index)? {
            ConstantInfo::Class(ref s) => Ok(self.constant_utf8(s.name_index)?),
            _ => Err("Not a class constant".to_owned()),
        }
    }

    fn constant_name_and_type(&self, index: u16) -> Result<(&str, &str), String> {
        match *self.constant(index)? {
            ConstantInfo::NameAndType(ref s) => {
                Ok((self.constant_utf8(s.name_index)?, self.constant_utf8(s.descriptor_index)?))
            }
            _ => Err("Not a class constant".to_owned()),
        }
    }

    fn constant_field_ref(&self, index: u16) -> Result<FieldRef, String> {
        match *self.constant(index)? {
            ConstantInfo::FieldRef(ref s) => {
                let (name, typ) = self.constant_name_and_type(s.name_and_type_index)?;
                Ok(FieldRef {
                    class: self.constant_class(s.class_index)?.to_owned(),
                    name: name.to_owned(),
                    descriptor: typ.to_owned(),
                })
            }
            _ => Err("Not a class constant".to_owned()),
        }
    }

    fn constant_method_ref(&self, index: u16) -> Result<MethodRef, String> {
        match *self.constant(index)? {
            ConstantInfo::MethodRef(ref s) => {
                let (name, typ) = self.constant_name_and_type(s.name_and_type_index)?;
                Ok(MethodRef {
                    class: self.constant_class(s.class_index)?.to_owned(),
                    name: name.to_owned(),
                    descriptor: typ.to_owned(),
                })
            }
            _ => Err("Not a class constant".to_owned()),
        }
    }

    fn constant_interface_method_ref(&self, index: u16) -> Result<MethodRef, String> {
        match *self.constant(index)? {
            ConstantInfo::InterfaceMethodRef(ref s) => {
                let (name, typ) = self.constant_name_and_type(s.name_and_type_index)?;
                Ok(MethodRef {
                    class: self.constant_class(s.class_index)?.to_owned(),
                    name: name.to_owned(),
                    descriptor: typ.to_owned(),
                })
            }
            _ => Err("Not a class constant".to_owned()),
        }
    }
}
