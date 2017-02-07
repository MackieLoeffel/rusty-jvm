
#[derive(Debug, PartialEq, Eq)]
pub struct FieldDescriptor {
    num_array: usize,
    typ: FieldDescriptorType,
}

#[derive(Debug, PartialEq, Eq)]
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
               FieldDescriptor {num_array: num_array.len(), typ: typ}
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

impl FieldDescriptor {
    pub fn parse(desc: &str) -> Option<FieldDescriptor> {
        named!(fd_eof<&str, FieldDescriptor>, do_parse!(
            fd: field_descriptor >>
                eof!() >> (fd)
               ));
        fd_eof(desc).to_result().ok()

    }
}

impl MethodDescriptor {
    pub fn parse(desc: &str) -> Option<MethodDescriptor> {
        named!(md_eof<&str, MethodDescriptor>, do_parse!(
            fd: method_descriptor >>
                eof!() >> (fd)
        ));
        md_eof(desc).to_result().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::FieldDescriptorType::*;

    fn fd(typ: FieldDescriptorType, num_array: usize) -> Option<FieldDescriptor> {
        Some(fdo(typ, num_array))
    }

    fn fdo(typ: FieldDescriptorType, num_array: usize) -> FieldDescriptor {
        FieldDescriptor {
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
    fn field_double_array() {
        assert_eq!(FieldDescriptor::parse("[[[[D"), fd(Double, 4));
    }

    #[test]
    fn field_fail() {
        assert_eq!(FieldDescriptor::parse("[[D[[D"), None);
    }

    #[test]
    fn method_empty() {
        assert_eq!(MethodDescriptor::parse("()V"), md(vec![], None));
    }

    #[test]
    fn method_one() {
        assert_eq!(MethodDescriptor::parse("(S)I"), md(vec![fdo(Byte, 0)], fd(Int, 0)));
    }

    #[test]
    fn method_many() {
        assert_eq!(MethodDescriptor::parse("(IDLjava/lang/Thread;)Ljava/lang/Object;"), md(vec![], None));
    }

    #[test]
    fn method_fail() {
        assert_eq!(MethodDescriptor::parse("()V()"), None);
    }
}
