use classfile_parser::{ClassFile, parse_class};
use classfile_parser::method_info::*;
use classfile_parser::attribute_info::*;
use instruction::Instruction;
use parsed_class::{ParsedClass};

// see https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html#40222
pub const MAX_INSTRUCTIONS_PER_METHOD: usize = 65536;

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

impl Class {
    pub fn from_class_file(parsed: &ClassFile) -> Result<Class, String> {
        let name = parsed.constant_class(parsed.this_class)?;
        let super_class = parsed.constant_class(parsed.super_class)?;

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
    pub fn method_by_name(&self, name: &str) -> Option<&Method> { self.methods.iter().find(|m| m.name() == name) }

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
                        None => return Err("invalid code attribute".to_owned()),
                    };

                    if code_attr.code.len() == 0 {
                        return Err("Code may not be empty".to_owned());
                    }

                    if code_attr.code.len() > MAX_INSTRUCTIONS_PER_METHOD {
                        return Err(format!("Code of method {} is bigger than the maximum of {} (size: {})",
                                           name,
                                           MAX_INSTRUCTIONS_PER_METHOD,
                                           code_attr.code.len()));
                    }

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
    pub fn code(&self) -> Option<&Code> { self.code.as_ref() }
}

impl Code {
    pub fn from_class_file(attr: &CodeAttribute, parsed: &ClassFile) -> Result<Code, String> {
        Ok(Code {
            max_stack: attr.max_stack as usize,
            max_locals: attr.max_locals as usize,
            code: Instruction::decode(&attr.code, parsed)?,
        })
    }

    pub fn max_stack(&self) -> usize { self.max_stack }
    pub fn max_locals(&self) -> usize { self.max_locals }
    pub fn code(&self) -> &Vec<Instruction> { &self.code }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_class() -> Class { Class::from_class_file(&parse_class("./assets/TestClass").unwrap()).unwrap() }

    #[test]
    fn name() {
        assert_eq!(get_class().name(), "com/mackie/rustyjvm/TestClass");
    }

    #[test]
    fn super_class() {
        assert_eq!(get_class().super_class(), "java/lang/Object");
    }

    #[test]
    fn method_init() {
        let class = get_class();
        assert_eq!(class.methods.len(), 2);
        let method = &class.methods[0];
        assert_eq!(method.name(), "<init>");
        assert_eq!(method.descriptor(), "()V");
        assert_eq!(method.access_flags(), PUBLIC);
    }

    #[test]
    fn method_main() {
        let class = get_class();
        let method = class.method_by_name("main").unwrap();
        assert_eq!(method.name(), "main");
        assert_eq!(method.descriptor(), "([Ljava/lang/String;)V");
        assert_eq!(method.access_flags(), PUBLIC | STATIC);
    }

    #[test]
    fn method_not_exists() {
        assert!(get_class().method_by_name("unknown method").is_none());
    }

    #[test]
    fn code_main() {
        let class = get_class();
        let code = class.methods[1].code().unwrap();
        assert_eq!(code.max_stack(), 1);
        assert_eq!(code.max_locals(), 2);
        assert_eq!(code.code().len(), 3);
    }
}
