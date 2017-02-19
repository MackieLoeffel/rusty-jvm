use std::ops::Deref;
use instruction::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldDescriptor {
    num_array: usize,
    typ: FieldDescriptorType,
    simple_typ: Type,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldDescriptorType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Reference(String),
    Short,
    Boolean,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MethodDescriptor {
    params: Vec<FieldDescriptor>,
    ret_type: Option<FieldDescriptor>,
}

named!(field_descriptor<&str, FieldDescriptor>,
       do_parse!(
           num_array: many0!(tag!("[")) >>
           typ: alt!(
               tag!("B") => { |_| FieldDescriptorType::Byte }
               | tag!("C") => { |_| FieldDescriptorType::Char }
               | tag!("D") => { |_| FieldDescriptorType::Double }
               | tag!("F") => { |_| FieldDescriptorType::Float }
               | tag!("I") => { |_| FieldDescriptorType::Int }
               | tag!("J") => { |_| FieldDescriptorType::Long }
               | tag!("S") => { |_| FieldDescriptorType::Short }
               | tag!("Z") => { |_| FieldDescriptorType::Boolean }
               | do_parse!(
                   tag!("L") >>
                   name: take_until_and_consume!(";") >> ( FieldDescriptorType::Reference(name.to_owned()) )
               )
           ) >> (
               FieldDescriptor {num_array: num_array.len(),
                                simple_typ: as_type(&typ, num_array.len()),
                                typ: typ,
               }
           )
       )
);

named!(method_descriptor<&str, MethodDescriptor>,
       do_parse!(
           tag!("(") >>
           params: many0!(field_descriptor) >>
           tag!(")") >>
           ret_type: alt!(
               tag!("V") => { |_| None }
               | field_descriptor => {|fd| Some(fd)}
           ) >> (
               MethodDescriptor {params: params, ret_type: ret_type}
           )
       )
);

fn as_type(typ: &FieldDescriptorType, num_array: usize) -> Type {
    if num_array > 0 {
        return Type::Reference;
    }

    match *typ {
        FieldDescriptorType::Byte => Type::Byte,
        FieldDescriptorType::Char => Type::Char,
        FieldDescriptorType::Double => Type::Double,
        FieldDescriptorType::Float => Type::Float,
        FieldDescriptorType::Int => Type::Int,
        FieldDescriptorType::Long => Type::Long,
        FieldDescriptorType::Reference(..) => Type::Reference,
        FieldDescriptorType::Short => Type::Short,
        FieldDescriptorType::Boolean => Type::Boolean,
    }
}

impl FieldDescriptor {
    pub fn parse(desc: &str) -> Option<FieldDescriptor> {
        named!(fd_eof<&str, FieldDescriptor>, do_parse!(
            fd: field_descriptor >>
                eof!() >> (fd)
               ));
        fd_eof(desc).to_result().ok()
    }

    pub fn add_array(&mut self) {
        self.num_array += 1;
        self.update_simple_typ();
    }
    pub fn remove_array(&mut self) {
        assert!(self.num_array > 0);
        self.num_array -= 1;
        self.update_simple_typ();
    }

    // TODO remove
    #[allow(dead_code)]
    pub fn as_type_without_arrays(&self, num_less_arrays: usize) -> Type {
        assert!(num_less_arrays <= self.num_array);
        as_type(&self.typ, self.num_array - num_less_arrays)
    }

    pub fn simple_typ(&self) -> Type { self.simple_typ }
    pub fn is_array(&self) -> bool { self.num_array > 0 }

    pub fn get_class(&self) -> Option<&str> {
        if self.num_array > 0 {
            return None;
        }
        match self.typ {
            FieldDescriptorType::Reference(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn from_class(class: &str) -> FieldDescriptor {
        FieldDescriptor {
            num_array: 0,
            simple_typ: Type::Reference,
            typ: FieldDescriptorType::Reference(class.to_owned()),
        }
    }

    pub fn from_type_without_reference(typ: Type) -> FieldDescriptor {
        FieldDescriptor {
            num_array: 0,
            simple_typ: typ,
            typ: match typ {
                Type::Byte => FieldDescriptorType::Byte,
                Type::Char => FieldDescriptorType::Char,
                Type::Double => FieldDescriptorType::Double,
                Type::Float => FieldDescriptorType::Float,
                Type::Int => FieldDescriptorType::Int,
                Type::Long => FieldDescriptorType::Long,
                Type::Short => FieldDescriptorType::Short,
                Type::Boolean => FieldDescriptorType::Boolean,
                Type::Reference => panic!("reference cannot be converted to fieldDescriptor!"),
            },
        }
    }

    /// for references in ClassInfo structures
    /// see https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html#1221
    /// can be a classname or an array type
    pub fn from_symbolic_reference(name: &str) -> Option<FieldDescriptor> {
        if name.starts_with('[') {
            FieldDescriptor::parse(name)
        } else {
            Some(FieldDescriptor::from_class(name))
        }
    }

    fn update_simple_typ(&mut self) { self.simple_typ = as_type(&self.typ, self.num_array); }
}

impl Deref for FieldDescriptor {
    type Target = Type;

    fn deref(&self) -> &Type { &self.simple_typ }
}

impl MethodDescriptor {
    pub fn parse(desc: &str) -> Option<MethodDescriptor> {
        named!(md_eof<&str, MethodDescriptor>, do_parse!(
            fd: method_descriptor >>
                eof!() >> (fd)
        ));
        md_eof(desc).to_result().ok()
    }

    pub fn words_for_params(&self) -> usize { self.params.iter().map(|e| e.word_size()).sum() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::FieldDescriptorType::*;

    fn fd(typ: FieldDescriptorType, num_array: usize) -> Option<FieldDescriptor> { Some(fdo(typ, num_array)) }

    fn fdo(typ: FieldDescriptorType, num_array: usize) -> FieldDescriptor {
        FieldDescriptor {
            simple_typ: as_type(&typ, num_array),
            typ: typ,
            num_array: num_array,
        }
    }

    fn md(params: Vec<FieldDescriptor>, ret_type: Option<FieldDescriptor>) -> Option<MethodDescriptor> {
        Some(MethodDescriptor {
            params: params,
            ret_type: ret_type,
        })
    }

    #[test]
    fn field_byte() {
        assert_eq!(FieldDescriptor::parse("B"), fd(Byte, 0));
    }

    #[test]
    fn field_char() {
        assert_eq!(FieldDescriptor::parse("C"), fd(Char, 0));
    }

    #[test]
    fn field_double() {
        assert_eq!(FieldDescriptor::parse("D"), fd(Double, 0));
    }

    #[test]
    fn field_float() {
        assert_eq!(FieldDescriptor::parse("F"), fd(Float, 0));
    }

    #[test]
    fn field_int() {
        assert_eq!(FieldDescriptor::parse("I"), fd(Int, 0));
    }

    #[test]
    fn field_long() {
        assert_eq!(FieldDescriptor::parse("J"), fd(Long, 0));
    }

    #[test]
    fn field_short() {
        assert_eq!(FieldDescriptor::parse("S"), fd(Short, 0));
    }

    #[test]
    fn field_boolean() {
        assert_eq!(FieldDescriptor::parse("Z"), fd(Boolean, 0));
    }

    #[test]
    fn field_reference() {
        assert_eq!(FieldDescriptor::parse("Ljava/lang/Object;"),
                   fd(Reference("java/lang/Object".to_owned()), 0));
    }

    #[test]
    fn field_reference_array() {
        assert_eq!(FieldDescriptor::parse("[[Ljava/lang/Object;"),
                   fd(Reference("java/lang/Object".to_owned()), 2));
    }

    #[test]
    fn field_as_typ_without_array0() {
        assert_eq!(FieldDescriptor::parse("[[J").unwrap().as_type_without_arrays(0),
                   Type::Reference);
    }

    #[test]
    fn field_as_typ_without_array1() {
        assert_eq!(FieldDescriptor::parse("[[J").unwrap().as_type_without_arrays(1),
                   Type::Reference);
    }

    #[test]
    fn field_as_typ_without_array2() {
        assert_eq!(FieldDescriptor::parse("[[J").unwrap().as_type_without_arrays(2),
                   Type::Long);
    }

    #[test]
    fn field_double_array() {
        assert_eq!(FieldDescriptor::parse("[[[[D"), fd(Double, 4));
    }

    #[test]
    fn field_fail() {
        assert_eq!(FieldDescriptor::parse("[[D[[D"), None);
    }

    #[test]
    fn field_from_symolic_reference() {
        assert_eq!(FieldDescriptor::from_symbolic_reference("[Ljava/lang/Object;"),
                   fd(Reference("java/lang/Object".to_owned()), 1));
        assert_eq!(FieldDescriptor::from_symbolic_reference("java/lang/Object"),
                   fd(Reference("java/lang/Object".to_owned()), 0));
        assert_eq!(FieldDescriptor::from_symbolic_reference("[I"), fd(Int, 1));
    }

    #[test]
    fn method_empty() {
        assert_eq!(MethodDescriptor::parse("()V"), md(vec![], None));
    }

    #[test]
    fn method_one() {
        assert_eq!(MethodDescriptor::parse("(S)I"),
                   md(vec![fdo(Short, 0)], fd(Int, 0)));
    }

    #[test]
    fn method_many() {
        assert_eq!(MethodDescriptor::parse("(I[DLjava/lang/Thread;)Ljava/lang/Object;"),
                   md(vec![fdo(Int, 0), fdo(Double, 1), fdo(Reference("java/lang/Thread".to_owned()), 0)],
                      Some(fdo(Reference("java/lang/Object".to_owned()), 0))));
    }

    #[test]
    fn method_fail() {
        assert_eq!(MethodDescriptor::parse("()V()"), None);
    }

    #[test]
    fn method_words() {
        assert_eq!(MethodDescriptor::parse("(S[DJ)I").unwrap().words_for_params(),
                   4);
    }

    #[test]
    fn add_array() {
        let mut desc = FieldDescriptor::parse("J").unwrap();
        assert_eq!(desc.simple_typ(), Type::Long);
        assert_eq!(desc.num_array, 0);
        desc.add_array();
        assert_eq!(desc.simple_typ(), Type::Reference);
        assert_eq!(desc.num_array, 1);
    }

    #[test]
    fn remove_array() {
        let mut desc = FieldDescriptor::parse("[J").unwrap();
        assert_eq!(desc.simple_typ(), Type::Reference);
        assert_eq!(desc.num_array, 1);
        desc.remove_array();
        assert_eq!(desc.simple_typ(), Type::Long);
        assert_eq!(desc.num_array, 0);
    }

    #[test]
    fn get_class() {
        assert_eq!(FieldDescriptor::parse("J").unwrap().get_class(), None);
        assert_eq!(FieldDescriptor::parse("[LA;").unwrap().get_class(), None);
        assert_eq!(FieldDescriptor::parse("LA;").unwrap().get_class(),
                   Some("A"));
    }
}
