use classfile_parser;
use classfile_parser::{ClassFile, method_info, field_info, ClassAccessFlags};
use classfile_parser::method_info::*;
use classfile_parser::field_info::*;
use classfile_parser::attribute_info::*;
use instruction::{Instruction, Type};
use parsed_class::{ParsedClass, FieldRef};
use descriptor::{MethodDescriptor, FieldDescriptor};
use class_loader::ClassLoader;
use errors::ClassLoadingError;

// see https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html#40222
pub const MAX_INSTRUCTIONS_PER_METHOD: usize = 65536;
pub const OBJECT_NAME: &'static str = "java/lang/Object";
// interfaces, which an array implements:
// https://docs.oracle.com/javase/specs/jvms/se6/html/Concepts.doc.html#16446
pub const SERIALIZABLE_NAME: &'static str = "java/io/Serializable";
pub const CLONEABLE_NAME: &'static str = "java/lang/Cloneable";

#[derive(Debug)]
pub struct Class {
    name: String,
    super_class: Option<String>,
    interfaces: Vec<String>,
    access_flags: ClassAccessFlags,
    methods: Vec<Method>,
    static_fields: Vec<Field>,
    instance_fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Method {
    access_flags: MethodAccessFlags,
    name: String,
    descriptor: String,
    code: Option<Code>,
    words_for_params: usize,
}

#[derive(Debug)]
pub struct Field {
    access_flags: FieldAccessFlags,
    // TODO: think of a better type
    constant_value: Option<Result<[i32; 2], String>>,
    name: String,
    descriptor: String,
    size: usize,
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
        let super_class = if name == OBJECT_NAME {
            if parsed.super_class != 0 {
                return Err("Object must not have a superclass".to_owned());
            }
            None
        } else {
            if parsed.super_class == 0 {
                return Err("Non-Object-Class must have a superclass".to_owned());
            }
            let super_class_name = parsed.constant_class(parsed.super_class)?.to_owned();
            if parsed.access_flags.contains(classfile_parser::INTERFACE) && super_class_name != OBJECT_NAME {
                return Err("Interfaces must have Object as Superclass".to_owned());
            }
            Some(super_class_name)
        };

        let interfaces = parsed.interfaces
            .iter()
            .map(|index| parsed.constant_class(*index).map(|s| s.to_owned()))
            .collect::<Result<Vec<String>, String>>()?;

        let methods = parsed.methods
            .iter()
            .map(|info| Method::from_class_file(info, parsed))
            .collect::<Result<Vec<_>, String>>()?;

        let (static_fields, instance_fields) = parsed.fields
            .iter()
            .map(|info| Field::from_class_file(info, parsed))
            .collect::<Result<Vec<_>, String>>()?
            .into_iter()
            .partition(|f| f.is_static());

        Ok(Class {
            name: name.to_owned(),
            super_class: super_class,
            interfaces: interfaces,
            access_flags: parsed.access_flags,
            methods: methods,
            instance_fields: instance_fields,
            static_fields: static_fields,
        })
    }

    pub fn method_by_signature(&self, name: &str, descriptor: &str) -> Option<&Method> {
        self.methods.iter().find(|m| m.name() == name && m.descriptor() == descriptor)
    }

    // we can't extract the iterating part from these methods, because streaming
    // iterators are not possible with the curent Iterator interface
    // ses https://www.reddit.com/r/rust/comments/303a09/looking_for_more_information_on_streaming/
    pub fn get_instance_size(classname: &str, classloader: &mut ClassLoader) -> Result<usize, ClassLoadingError> {
        let mut cur_name = classname.to_owned();
        let mut sum = 0;
        loop {
            let class = classloader.load_class(&cur_name)?;
            sum += class.instance_fields().iter().map(|f| f.size()).sum();
            cur_name = match class.super_class() {
                Some(c) => c.to_owned(),
                None => break,
            }
        }
        Ok(sum)
    }

    /// calculates the offset of a specific field in a class
    /// the class in fieldref is the class, for which the offset should
    /// be calculated, not the class with the field definiton
    /// the first variable in Object has offset 0, the last variable in the current
    /// class has the biggest offset. This is to ensure, that upcasting works correctly
    /// and we don't have to do anything there
    /// TODO think about handling static fields, which shadow instance fields
    pub fn get_field_offset(fieldref: &FieldRef, classloader: &mut ClassLoader) -> Result<usize, ClassLoadingError> {
        let mut cur_name = fieldref.class().to_owned();
        // None, if we didn't find the field yet
        //  otherwise contains the current offset up to this superclass
        let mut offset = None;
        loop {
            let class = classloader.load_class(&cur_name)?;
            let mut current_offset = 0;
            let mut found = false;
            for field in class.instance_fields() {
                if offset.is_none() && field.name() == fieldref.name() && field.descriptor() == fieldref.descriptor() {
                    found = true;
                    break;
                }
                current_offset += field.size()
            }
            if found {
                assert!(offset.is_none());
                offset = Some(current_offset);
            } else {
                offset = offset.map(|v| v + current_offset);
            }
            cur_name = match class.super_class() {
                Some(c) => c.to_owned(),
                None => break,
            }
        }
        match offset {
            Some(o) => Ok(o),
            None => Err(ClassLoadingError::NoSuchFieldError(fieldref.clone())),
        }
    }

    pub fn is_real_super_class(superclass: &str,
                               class: &str,
                               classloader: &mut ClassLoader)
                               -> Result<bool, ClassLoadingError> {
        let mut cur_name = class.to_owned();
        loop {
            let class = classloader.load_class(&cur_name)?;
            cur_name = match class.super_class() {
                Some(c) => c.to_owned(),
                None => return Ok(false),
            };
            if cur_name == superclass {
                return Ok(true);
            }
        }
    }

    pub fn find_first_real_super_class_with_method(classname: &str,
                                                   name: &str,
                                                   descriptor: &str,
                                                   classloader: &mut ClassLoader)
                                                   -> Result<Option<String>, ClassLoadingError> {
        let super_name;
        {
            let class = classloader.load_class(classname)?;
            super_name = match class.super_class() {
                Some(c) => c.to_owned(),
                None => return Ok(None),
            }
        }
        Class::find_first_super_class_with_method(&super_name, name, descriptor, classloader)
    }
    pub fn find_first_super_class_with_method(class: &str,
                                              name: &str,
                                              descriptor: &str,
                                              classloader: &mut ClassLoader)
                                              -> Result<Option<String>, ClassLoadingError> {
        let mut cur_name = class.to_owned();
        loop {
            let class = classloader.load_class(&cur_name)?;
            if class.method_by_signature(name, descriptor).is_some() {
                return Ok(Some(cur_name));
            }
            cur_name = match class.super_class() {
                Some(c) => c.to_owned(),
                None => return Ok(None),
            };
        }
    }

    pub fn is_instance_of(class: &FieldDescriptor,
                          mut dest: FieldDescriptor,
                          classloader: &mut ClassLoader)
                          -> Result<bool, ClassLoadingError> {
        // see logic at https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc6.html under instanceof
        if class.is_array() {
            if dest.is_array() {
                let mut new_class = class.clone();
                new_class.remove_array();
                dest.remove_array();
                if new_class.simple_typ() == Type::Reference {
                    Class::is_instance_of(&new_class, dest, classloader)
                } else {
                    Ok(new_class.simple_typ() == dest.simple_typ())
                }
            } else {
                let dest_class = match dest.get_class() {
                    Some(s) => s,
                    None => return Ok(false),
                };
                Ok(dest_class == OBJECT_NAME || dest_class == CLONEABLE_NAME || dest_class == SERIALIZABLE_NAME)
            }
        } else {
            // interfaces have object as superclass, so we can merge the interface and class cases
            let class_name = class.get_class().expect("is_instance_of must be called with references (class)");
            let dest_name = dest.get_class().expect("is_instance_of must be called with references (dest)");
            if class_name == dest_name {
                return Ok(true);
            }
            if Class::has_interface_or_superclass(class_name, dest_name, classloader)? {
                return Ok(true);
            }
            Ok(false)
        }
    }

    pub fn has_interface_or_superclass(class: &str,
                                       super_name: &str,
                                       classloader: &mut ClassLoader)
                                       -> Result<bool, ClassLoadingError> {
        if class == super_name {
            return Ok(true);
        }

        let mut interfaces;
        {
            let loaded_class = classloader.load_class(class)?;
            interfaces = loaded_class.interfaces.clone();
            if let Some(s) = loaded_class.super_class() {
                interfaces.push(s.to_owned());
            }
        }

        while !interfaces.is_empty() {
            let interface = classloader.load_class(&interfaces.pop().unwrap())?;
            if interface.name() == super_name {
                return Ok(true);
            }
            interfaces.extend(interface.interfaces.clone());
            if let Some(s) = interface.super_class() {
                interfaces.push(s.to_owned());
            }
        }
        Ok(false)
    }

    pub fn name(&self) -> &str { &self.name }
    #[allow(dead_code)]
    pub fn methods(&self) -> &Vec<Method> { &self.methods }
    pub fn instance_fields(&self) -> &Vec<Field> { &self.instance_fields }
    #[allow(dead_code)]
    pub fn static_fields(&self) -> &Vec<Field> { &self.static_fields }
    pub fn super_class(&self) -> Option<&String> { self.super_class.as_ref() }
    pub fn has_acc_super_flag(&self) -> bool { self.access_flags.contains(classfile_parser::SUPER) }
}

impl Method {
    pub fn from_class_file(info: &MethodInfo, parsed: &ClassFile) -> Result<Method, String> {
        let name = parsed.constant_utf8(info.name_index)?;
        let descriptor = parsed.constant_utf8(info.descriptor_index)?;

        let mut code: Option<Code> = None;
        for attr in &info.attributes {
            match parsed.constant_utf8(attr.attribute_name_index)? {
                "Code" => {
                    if code.is_some() {
                        return Err("two code attributes".to_owned());
                    }

                    let code_attr = match attr.try_as_code_attribute() {
                        Some(c) => c,
                        None => return Err("invalid code attribute".to_owned()),
                    };

                    if code_attr.code.is_empty() {
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

        let parsed_descriptor = match MethodDescriptor::parse(descriptor) {
            Some(c) => c,
            None => return Err(format!("invalid method descriptor for method {}", name)),
        };

        let mut words_for_params = parsed_descriptor.words_for_params();
        if !info.access_flags.contains(method_info::STATIC) {
            words_for_params += Type::Reference.word_size()
        };

        Ok(Method {
            access_flags: info.access_flags,
            name: name.to_owned(),
            descriptor: descriptor.to_owned(),
            code: code,
            words_for_params: words_for_params,
        })
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn descriptor(&self) -> &str { &self.descriptor }
    pub fn access_flags(&self) -> MethodAccessFlags { self.access_flags }
    pub fn code(&self) -> Option<&Code> { self.code.as_ref() }
    pub fn words_for_params(&self) -> usize { self.words_for_params }
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

impl Field {
    pub fn from_class_file(info: &FieldInfo, parsed: &ClassFile) -> Result<Field, String> {
        let name = parsed.constant_utf8(info.name_index)?;
        let descriptor = parsed.constant_utf8(info.descriptor_index)?;

        let parsed_descriptor = match FieldDescriptor::parse(descriptor) {
            Some(c) => c,
            None => return Err(format!("invalid field descriptor for field {}", name)),
        };

        Ok(Field {
            access_flags: info.access_flags,
            name: name.to_owned(),
            descriptor: descriptor.to_owned(),
            size: parsed_descriptor.word_size(),
            constant_value: None, // TODO
        })
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str { &self.name }
    #[allow(dead_code)]
    pub fn descriptor(&self) -> &str { &self.descriptor }
    pub fn size(&self) -> usize { self.size }
    pub fn is_static(&self) -> bool { self.access_flags.contains(field_info::STATIC) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use classfile_parser::parse_class;


    fn get_class() -> Class {
        Class::from_class_file(&parse_class(&(super::super::CLASSFILE_DIR.to_owned() + "/TestClass")).unwrap()).unwrap()
    }

    #[test]
    fn name() {
        assert_eq!(get_class().name(), "com/mackie/rustyjvm/TestClass");
    }

    #[test]
    fn acc_super() {
        assert_eq!(get_class().has_acc_super_flag(), true);
    }

    #[test]
    fn super_class() {
        assert_eq!(get_class().super_class().unwrap(),
                   "com/mackie/rustyjvm/TestClassSuper");
    }

    #[test]
    fn method_init() {
        let class = get_class();
        assert_eq!(class.methods.len(), 3);
        let method = &class.methods[0];
        assert_eq!(method.name(), "<init>");
        assert_eq!(method.descriptor(), "()V");
        assert_eq!(method.access_flags(), method_info::PUBLIC);
    }

    #[test]
    fn method_main() {
        let class = get_class();
        let method = class.method_by_signature("main", "([Ljava/lang/String;)V").unwrap();
        assert_eq!(method.name(), "main");
        assert_eq!(method.descriptor(), "([Ljava/lang/String;)V");
        assert_eq!(method.access_flags(),
                   method_info::PUBLIC | method_info::STATIC);
    }

    #[test]
    fn method_not_exists() {
        assert!(get_class().method_by_signature("unknown method", "").is_none());
    }

    #[test]
    fn code_main() {
        let class = get_class();
        let code = class.methods[1].code().unwrap();
        assert_eq!(code.max_stack(), 1);
        assert_eq!(code.max_locals(), 2);
        assert_eq!(code.code().len(), 3);
    }

    #[test]
    fn fields_size() {
        let class = get_class();
        assert_eq!(class.instance_fields().len(), 3);
        assert_eq!(class.instance_fields()[0].name(), "a");
        assert_eq!(class.instance_fields()[0].descriptor(), "I");
        assert_eq!(class.static_fields().len(), 1);
        assert_eq!(class.static_fields()[0].name(), "c");
        assert_eq!(class.static_fields()[0].descriptor(), "S");
    }

    #[test]
    fn is_super_class() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        assert_eq!(Class::is_real_super_class("com/mackie/rustyjvm/TestClass",
                                              "com/mackie/rustyjvm/TestClass",
                                              &mut classloader)
                       .unwrap(),
                   false);
        assert_eq!(Class::is_real_super_class("com/mackie/rustyjvm/TestClass",
                                              "com/mackie/rustyjvm/TestClassSuper",
                                              &mut classloader)
                       .unwrap(),
                   false);
        assert_eq!(Class::is_real_super_class("com/mackie/rustyjvm/TestClassSuper",
                                              "com/mackie/rustyjvm/TestClass",
                                              &mut classloader)
                       .unwrap(),
                   true);
        assert_eq!(Class::is_real_super_class("java/lang/Object",
                                              "com/mackie/rustyjvm/TestClass",
                                              &mut classloader)
                       .unwrap(),
                   true);
    }

    #[test]
    fn field_offset() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        assert_eq!(Class::get_field_offset(&FieldRef::new("a", "com/mackie/rustyjvm/TestClass", "I").unwrap(),
                                           &mut classloader)
                       .unwrap(),
                   4);
        assert_eq!(Class::get_field_offset(&FieldRef::new("c", "com/mackie/rustyjvm/TestClass", "J").unwrap(),
                                           &mut classloader)
                       .unwrap(),
                   1);
        assert_eq!(Class::get_field_offset(&FieldRef::new("d", "com/mackie/rustyjvm/TestClass", "B").unwrap(),
                                           &mut classloader)
                       .unwrap(),
                   3);
        assert_eq!(Class::get_field_offset(&FieldRef::new("d", "com/mackie/rustyjvm/TestClass", "D").unwrap(),
                                           &mut classloader)
                       .unwrap(),
                   5);
        assert_eq!(Class::get_field_offset(&FieldRef::new("e", "com/mackie/rustyjvm/TestClass", "[D").unwrap(),
                                           &mut classloader)
                       .unwrap(),
                   7);
        assert_eq!(Class::get_field_offset(&FieldRef::new("c", "com/mackie/rustyjvm/TestClass", "S").unwrap(),
                                           &mut classloader)
                       .is_err(),
                   true);
    }


    #[test]
    fn field_size() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        assert_eq!(Class::get_instance_size("com/mackie/rustyjvm/TestClass", &mut classloader).unwrap(),
                   8);
    }

    #[test]
    fn find_methods() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        assert_eq!(Class::find_first_super_class_with_method("com/mackie/rustyjvm/TestClass",
                                                             "virtualMethod",
                                                             "()V",
                                                             &mut classloader)
                       .unwrap(),
                   Some("com/mackie/rustyjvm/TestClass".to_owned()));
        assert_eq!(Class::find_first_real_super_class_with_method("com/mackie/rustyjvm/TestClass",
                                                                  "virtualMethod",
                                                                  "()V",
                                                                  &mut classloader)
                       .unwrap(),
                   Some("com/mackie/rustyjvm/TestClassSuper".to_owned()));
        assert_eq!(Class::find_first_super_class_with_method("com/mackie/rustyjvm/TestClass",
                                                             "virtualMethod",
                                                             "(I)V",
                                                             &mut classloader)
                       .unwrap(),
                   None);


    }

    #[test]
    fn instance_of() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        macro_rules! check(
            ($class: expr, $dest: expr, $val: expr) => {{
                assert_eq!(Class::is_instance_of(&FieldDescriptor::parse($class).unwrap(),
                                                 FieldDescriptor::parse($dest).unwrap(),
                                                 &mut classloader).unwrap(), $val);
            }});
        check!("Lcom/mackie/rustyjvm/TestClass;",
               "Lcom/mackie/rustyjvm/TestClass;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClass;",
               "Lcom/mackie/rustyjvm/TestClassSuper;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClass;",
               "Ljava/lang/Object;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClassSuper;",
               "Lcom/mackie/rustyjvm/TestClass;",
               false);
        check!("Lcom/mackie/rustyjvm/TestClass;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClass;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceB;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClass;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceC;",
               false);
        check!("Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClassInterfaceB;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClassInterfaceC;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceC;",
               false);
        check!("Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               "Ljava/lang/Object;",
               true);
        check!("Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               "Lcom/mackie/rustyjvm/TestClass;",
               false);
        check!("[Lcom/mackie/rustyjvm/TestClassSuper;",
               "Ljava/lang/Object;",
               true);
        check!("[Lcom/mackie/rustyjvm/TestClassSuper;",
               "Lcom/mackie/rustyjvm/TestClass;",
               false);
        check!("[Lcom/mackie/rustyjvm/TestClassSuper;",
               "[Lcom/mackie/rustyjvm/TestClassSuper;",
               true);
        check!("[Lcom/mackie/rustyjvm/TestClassSuper;",
               "[Ljava/lang/Object;",
               true);
        check!("[[[I", "[[[I", true);
        check!("[[[I", "[[I", false);
        check!("[[[I", "[[[D", false);
        check!("[[[I", "Ljava/lang/Cloneable;", true);
        check!("[[[I", "Ljava/io/Serializable;", true);
        check!("[Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               "Lcom/mackie/rustyjvm/TestClassInterfaceA;",
               false);
    }

    #[test]
    fn has_interface() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        macro_rules! check(
            ($interface: expr, $class: expr, $val: expr) => {{
                assert_eq!(Class::has_interface_or_superclass($interface, $class,
                                                              &mut classloader).unwrap(),
                           $val);
            }});

        check!("com/mackie/rustyjvm/TestClass",
               "com/mackie/rustyjvm/TestClassInterfaceA",
               true);
        check!("com/mackie/rustyjvm/TestClass",
               "com/mackie/rustyjvm/TestClassInterfaceB",
               true);
        check!("com/mackie/rustyjvm/TestClass",
               "com/mackie/rustyjvm/TestClassInterfaceC",
               false);
        check!("com/mackie/rustyjvm/TestClass", "java/lang/Object", true);
        check!("com/mackie/rustyjvm/TestClassSuper",
               "com/mackie/rustyjvm/TestClass",
               false);
        check!("com/mackie/rustyjvm/TestClassInterfaceA",
               "com/mackie/rustyjvm/TestClassInterfaceA",
               true);
        check!("com/mackie/rustyjvm/TestClassInterfaceB",
               "com/mackie/rustyjvm/TestClassInterfaceA",
               true);
        check!("com/mackie/rustyjvm/TestClassInterfaceC",
               "com/mackie/rustyjvm/TestClassInterfaceA",
               true);
        check!("com/mackie/rustyjvm/TestClassInterfaceA",
               "com/mackie/rustyjvm/TestClassInterfaceC",
               false);
        check!("com/mackie/rustyjvm/TestClassInterfaceA",
               "java/lang/Object",
               true);
        check!("com/mackie/rustyjvm/TestClassInterfaceA",
               "com/mackie/rustyjvm/TestClass",
               false);
    }
}
