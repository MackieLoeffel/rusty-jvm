use classfile_parser::*;
use classfile_parser::constant_info::*;

#[derive(Debug)]
pub struct Class {
    name: String,
    super_class: String,
}

impl Class {
    pub fn from_class_file(parsed: &ClassFile) -> Result<Class, String> {
        let this_class = parsed.constant_class(parsed.this_class)?;
        let name = parsed.constant_utf8(this_class.name_index)?;
        let super_class =
            parsed.constant_utf8(parsed.constant_class(parsed.super_class)?.name_index)?;

        Ok(Class {
            name: name,
            super_class: super_class,
        })
    }

    pub fn name(&self) -> &str { &self.name }
    #[allow(dead_code)]
    pub fn super_class(&self) -> &str { &self.super_class }
}

trait ParsedClass {
    fn constant(&self, index: u16) -> Result<&ConstantInfo, String>;
    fn constant_utf8(&self, index: u16) -> Result<String, String>;
    fn constant_class(&self, index: u16) -> Result<&ClassConstant, String>;
}

impl ParsedClass for ClassFile {
    fn constant(&self, index: u16) -> Result<&ConstantInfo, String> {
        if index == 0 || index as usize > self.const_pool.len() {
            return Err("index out of bounds".to_owned());
        }
        return Ok(&self.const_pool[(index - 1) as usize]);
    }

    fn constant_utf8(&self, index: u16) -> Result<String, String> {
        match *self.constant(index)? {
            ConstantInfo::Utf8(ref s) => Ok(s.utf8_string.clone()),
            _ => Err("Not a utf8 constant".to_owned()),
        }
    }

    fn constant_class(&self, index: u16) -> Result<&ClassConstant, String> {
        match *self.constant(index)? {
            ConstantInfo::Class(ref s) => Ok(s),
            _ => Err("Not a class constant".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_class(classname: &str) -> Class {
        Class::from_class_file(&parse_class(&format!("./assets/{}", classname)).unwrap()).unwrap()
    }

    #[test]
    fn name() {
        assert_eq!(get_class("SimpleClass").name(),
                   "com/mackie/rustyjvm/SimpleClass");
    }

    #[test]
    fn super_class() {
        assert_eq!(get_class("SimpleClass").super_class(), "java/lang/Object");
    }
}
