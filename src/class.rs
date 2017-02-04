use classfile_parser::*;
use classfile_parser::constant_info::*;
use classfile_parser::method_info::*;
use classfile_parser::attribute_info::*;

#[derive(Debug)]
pub struct Class {
    name: String,
    super_class: String,
    methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Method {
    access_flags: MethodAccessFlags,
    name: String,
    descriptor: String,
    code: Option<Code>,
}

#[derive(Debug)]
pub struct Code {
    // TODO exception table
    max_stack: usize,
    max_locals: usize,
    code: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    UnknownInstruction,
}

impl Class {
    pub fn from_class_file(parsed: &ClassFile) -> Result<Class, String> {
        let this_class = parsed.constant_class(parsed.this_class)?;
        let name = parsed.constant_utf8(this_class.name_index)?;
        let super_class =
            parsed.constant_utf8(parsed.constant_class(parsed.super_class)?.name_index)?;

        let methods = parsed.methods
            .iter()
            .map(|info| Method::from_class_file(info, parsed))
            .collect::<Result<Vec<Method>, String>>()?;

        Ok(Class {
            name: name.to_owned(),
            super_class: super_class.to_owned(),
            methods: methods,
        })
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str { &self.name }
    #[allow(dead_code)]
    pub fn super_class(&self) -> &str { &self.super_class }
}

impl Method {
    pub fn from_class_file(info: &MethodInfo, parsed: &ClassFile) -> Result<Method, String> {
        let name = parsed.constant_utf8(info.name_index)?;
        let descriptor = parsed.constant_utf8(info.descriptor_index)?;

        let mut code: Option<Code> = None;
        for attr in info.attributes.iter() {
            match parsed.constant_utf8(attr.attribute_name_index)? {
                "Code" => {
                    if code.is_some() {
                        return Err("two code attributes".to_owned());
                    }

                    let code_attr = match attr.try_as_code_attribute() {
                        Some(c) => c,
                        None => return Err("invalid code attributes".to_owned()),
                    };
                    code = Some(Code::from_class_file(&code_attr, parsed)?)
                }
                // ignore unknown attributes, see spec
                _ => {}
            };
        }

        Ok(Method {
            access_flags: info.access_flags,
            name: name.to_owned(),
            descriptor: descriptor.to_owned(),
            code: code,
        })
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str { &self.name }
    #[allow(dead_code)]
    pub fn descriptor(&self) -> &str { &self.descriptor }
    #[allow(dead_code)]
    pub fn access_flags(&self) -> MethodAccessFlags { self.access_flags }
    #[allow(dead_code)]
    pub fn code(&self) -> &Option<Code> { &self.code }
}

impl Code {
    pub fn from_class_file(attr: &CodeAttribute, _parsed: &ClassFile) -> Result<Code, String> {
        Ok(Code {
            max_stack: attr.max_stack as usize,
            max_locals: attr.max_locals as usize,
            // TODO: use real instructions
            code: attr.code.iter().map(|_b| Instruction::UnknownInstruction).collect(),
        })
    }

    #[allow(dead_code)]
    pub fn max_stack(&self) -> usize { self.max_stack }
    #[allow(dead_code)]
    pub fn max_locals(&self) -> usize { self.max_locals }
    #[allow(dead_code)]
    pub fn code(&self) -> &Vec<Instruction> { &self.code }
}

trait ParsedClass {
    fn constant(&self, index: u16) -> Result<&ConstantInfo, String>;
    fn constant_utf8(&self, index: u16) -> Result<&str, String>;
    fn constant_class(&self, index: u16) -> Result<&ClassConstant, String>;
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

    #[test]
    fn method_init() {
        let class = get_class("SimpleClass");
        assert_eq!(class.methods.len(), 2);
        let method = &class.methods[0];
        assert_eq!(method.name(), "<init>");
        assert_eq!(method.descriptor(), "()V");
        assert_eq!(method.access_flags(), PUBLIC);
    }

    #[test]
    fn method_main() {
        let method = &get_class("SimpleClass").methods[1];
        assert_eq!(method.name(), "main");
        assert_eq!(method.descriptor(), "([Ljava/lang/String;)V");
        assert_eq!(method.access_flags(), PUBLIC | STATIC);
    }

    #[test]
    fn code_main() {
        let class = get_class("SimpleClass");
        let code = class.methods[1].code().as_ref().unwrap();
        assert_eq!(code.max_stack(), 1);
        assert_eq!(code.max_locals(), 2);
        assert_eq!(code.code().len(), 3);
    }
}
