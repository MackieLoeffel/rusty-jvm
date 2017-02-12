use instruction::Type;

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
pub struct InstanceObject {}

impl Object {
    pub fn new_array(length: i32, typ: Type) -> Object { Object::Array(ArrayObject::new(length, typ)) }

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
        [self.data[index as usize], self.data[index as usize + 1]]
    }

    pub fn set(&mut self, index: i32, val: i32) {
        assert!(!self.content_needs_two_words);
        self.data[index as usize] = val;
    }
    pub fn set2(&mut self, index: i32, val: [i32; 2]) {
        assert!(self.content_needs_two_words);
        self.data[index as usize] = val[0];
        self.data[index as usize + 1] = val[1];
    }
}