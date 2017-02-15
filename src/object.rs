use class_loader::ClassLoader;
use instruction::Type;
use errors::ClassLoadingError;
use class::Class;
use parsed_class::FieldRef;

#[derive(Debug)]
pub enum Object {
    Array(ArrayObject),
    #[allow(dead_code)]
    Instance(InstanceObject),
}

#[derive(Debug)]
pub struct ArrayObject {
    length: i32,
    content_needs_two_words: bool,
    data: Box<[i32]>,
}

#[derive(Debug)]
pub struct InstanceObject {
    class: String,
    data: Box<[i32]>,
}

impl Object {
    pub fn new_array(length: i32, typ: Type) -> Object { Object::Array(ArrayObject::new(length, typ)) }

    pub fn new_instance(class: &str, class_loader: &mut ClassLoader) -> Result<Object, ClassLoadingError> {
        Ok(Object::Instance(InstanceObject::new(class, class_loader)?))
    }

    pub fn as_array(&mut self) -> &mut ArrayObject {
        match *self {
            Object::Array(ref mut a) => a,
            _ => panic!("expected array"),
        }
    }

    #[allow(dead_code)]
    pub fn as_instance(&mut self) -> &mut InstanceObject {
        match *self {
            Object::Instance(ref mut a) => a,
            _ => panic!("expected array"),
        }
    }
}

impl ArrayObject {
    pub fn new(length: i32, typ: Type) -> ArrayObject {
        let cap = (length as usize) * typ.word_size();
        let mut data = Vec::with_capacity(cap);
        data.resize(cap, 0);

        ArrayObject {
            length: length,
            content_needs_two_words: typ.is_double_sized(),
            data: data.into_boxed_slice(),
        }
    }

    pub fn length(&self) -> i32 { self.length }

    pub fn get(&self, index: i32) -> i32 {
        assert!(!self.content_needs_two_words);
        self.data[index as usize]
    }
    pub fn get2(&self, index: i32) -> [i32; 2] {
        assert!(self.content_needs_two_words);
        [self.data[2 * (index as usize)], self.data[2 * (index as usize) + 1]]
    }

    pub fn set(&mut self, index: i32, val: i32) {
        assert!(!self.content_needs_two_words);
        self.data[index as usize] = val;
    }
    pub fn set2(&mut self, index: i32, val: [i32; 2]) {
        assert!(self.content_needs_two_words);
        self.data[2 * (index as usize)] = val[0];
        self.data[2 * (index as usize) + 1] = val[1];
    }
}

impl InstanceObject {
    fn new(classname: &str, classloader: &mut ClassLoader) -> Result<InstanceObject, ClassLoadingError> {
        let len = Class::get_instance_size(classname, classloader)?;
        let mut data = Vec::with_capacity(len);
        data.resize(len, 0);
        Ok(InstanceObject {
            class: classname.to_owned(),
            data: data.into_boxed_slice(),
        })
    }

    pub fn get_field(&self, fieldref: &FieldRef, classloader: &mut ClassLoader) -> Result<i32, ClassLoadingError> {
        Ok(self.data[Class::get_field_offset(fieldref, classloader)?])
    }

    pub fn get_field2(&self,
                      fieldref: &FieldRef,
                      classloader: &mut ClassLoader)
                      -> Result<[i32; 2], ClassLoadingError> {
        let offset = Class::get_field_offset(fieldref, classloader)?;
        Ok([self.data[offset], self.data[offset + 1]])
    }

    pub fn set_field(&mut self,
                     fieldref: &FieldRef,
                     val: i32,
                     classloader: &mut ClassLoader)
                     -> Result<(), ClassLoadingError> {
        self.data[Class::get_field_offset(fieldref, classloader)?] = val;
        Ok(())
    }

    pub fn set_field2(&mut self,
                      fieldref: &FieldRef,
                      val: [i32; 2],
                      classloader: &mut ClassLoader)
                      -> Result<(), ClassLoadingError> {
        let offset = Class::get_field_offset(fieldref, classloader)?;
        self.data[offset] = val[0];
        self.data[offset + 1] = val[1];
        Ok(())
    }

    pub fn class(&self) -> &str { &self.class }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array() {
        let mut array = ArrayObject::new(3, Type::Long);
        array.set2(0, [1, 2]);
        array.set2(1, [3, 4]);
        assert_eq!(array.get2(0), [1, 2]);
        assert_eq!(array.get2(1), [3, 4]);
        assert_eq!(array.get2(2), [0, 0]);
    }

    #[test]
    fn instance() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        let mut instance = InstanceObject::new("com/mackie/rustyjvm/TestObject", &mut classloader).unwrap();
        assert_eq!(instance.data.len(), 8);
        assert_eq!(instance.get_field(&FieldRef::new("a", "com/mackie/rustyjvm/TestObject", "I").unwrap(),
                                  &mut classloader)
                       .unwrap(),
                   0);
        assert_eq!(instance.get_field2(&FieldRef::new("c", "com/mackie/rustyjvm/TestObject", "J").unwrap(),
                                   &mut classloader)
                       .unwrap(),
                   [0, 0]);
        assert_eq!(instance.set_field2(&FieldRef::new("c", "com/mackie/rustyjvm/TestObject", "J").unwrap(),
                                   [1, 2],
                                   &mut classloader)
                       .unwrap(),
                   ());
        assert_eq!(instance.get_field(&FieldRef::new("a", "com/mackie/rustyjvm/TestObject", "I").unwrap(),
                                  &mut classloader)
                       .unwrap(),
                   0);
        assert_eq!(instance.get_field2(&FieldRef::new("c", "com/mackie/rustyjvm/TestObject", "J").unwrap(),
                                   &mut classloader)
                       .unwrap(),
                   [1, 2]);
        assert_eq!(instance.set_field(&FieldRef::new("a", "com/mackie/rustyjvm/TestObject", "I").unwrap(),
                                  3,
                                  &mut classloader)
                       .unwrap(),
                   ());
        assert_eq!(instance.get_field(&FieldRef::new("a", "com/mackie/rustyjvm/TestObject", "I").unwrap(),
                                  &mut classloader)
                       .unwrap(),
                   3);
        assert_eq!(instance.get_field2(&FieldRef::new("c", "com/mackie/rustyjvm/TestObject", "J").unwrap(),
                                   &mut classloader)
                       .unwrap(),
                   [1, 2]);
    }

}
