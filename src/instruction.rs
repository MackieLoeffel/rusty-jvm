#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)] // TODO remove
pub enum Instruction {
    ALOAD(Type),
    ASTORE(Type),
    LOAD(Type, LocalVarRef),
    STORE(Type, LocalVarRef),

    ARRAYLENGTH,

    ATHROW,

    CHECKCAST(ConstPoolRef),
    INSTANCEOF(ConstPoolRef),

    ANEWARRAY(ConstPoolRef),
    MULTIANEWARRAY(ConstPoolRef, u8),
    NEW(ConstPoolRef),
    NEWARRAY(Type),

    // D2F, D2I,...
    CONVERT(Type, Type),

    ADD(Type),
    DIV(Type),
    MUL(Type),
    NEG(Type),
    REM(Type),
    SUB(Type),
    RETURN(Option<Type>),

    IINC(LocalVarRef, i16),

    AND(Type),
    OR(Type),
    SHL(Type),
    SHR(Type),
    USHR(Type),
    XOR(Type),

    DCMPG,
    DCMPL,
    FCMPG,
    FCMPL,
    LCMP,

    ACONST_NULL,
    DCONST_0,
    DCONST_1,
    FCONST_0,
    FCONST_1,
    FCONST_2,
    LCONST_0,
    LCONST_1,
    BIPUSH(i8),
    SIPUSH(i16),
    LDC(ConstPoolRef),
    LDC2_W(ConstPoolRef),

    DUP,
    DUP_X1,
    DUP_X2,
    DUP2,
    DUP2_X1,
    DUP2_X2,
    POP,
    POP2,
    SWAP,

    GETFIELD(ConstPoolRef),
    GETSTATIC(ConstPoolRef),
    PUTFIELD(ConstPoolRef),
    PUTSTATIC(ConstPoolRef),

    GOTO(i32),
    JSR(i32),
    RET(LocalVarRef),

    IF_ACMP(ComparisonEqual, i16),
    IF_ICMP(Comparison, i16),
    // comparison with zero
    IF(Comparison, i16),
    IFNULL(ComparisonEqual, i16),

    INVOKEINTERFACE(ConstPoolRef, u8),
    INVOKESPECIAL(ConstPoolRef),
    INVOKESTATIC(ConstPoolRef),
    INVOKEVIRTUAL(ConstPoolRef),

    LOOKUPSWITCH(i32, i32, Vec<(i32, i32)>),
    TABLESWITCH(i32, i32, i32, Vec<i32>),

    MONITORENTER,
    MONITOREXIT,

    NOP,
}

// index into the constant pool
pub type ConstPoolRef = u16;

// index into the local variables
pub type LocalVarRef = u16;

// true: equals, false: not equals
pub type ComparisonEqual = bool;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Reference,
    Char,
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Comparison {
    EQ,
    GE,
    GT,
    LE,
    LT,
    NE,
}

impl Instruction {
    pub fn decode(bytes: &[u8]) -> Result<Vec<Instruction>, String> {
        use self::Instruction::*;
        use self::Type::*;
        use self::Comparison::*;
        fn next(index: &mut usize, bytes: &[u8]) -> Result<u8, String> {
            if *index >= bytes.len() {
                return Err("incomplete instruction".to_owned());
            }
            *index += 1;
            Ok(bytes[*index - 1])
        }
        fn next_u16(index: &mut usize, bytes: &[u8]) -> Result<u16, String> {
            let b1 = next(index, bytes)? as u16;
            let b2 = next(index, bytes)? as u16;
            return Ok((b1 << 8) | b2);
        }
        fn next_u32(index: &mut usize, bytes: &[u8]) -> Result<u32, String> {
            let b1 = next_u16(index, bytes)? as u32;
            let b2 = next_u16(index, bytes)? as u32;
            return Ok((b1 << 16) | b2);
        }

        let mut vec = Vec::new();

        let mut index = 0;
        while index < bytes.len() {
            let cur = bytes[index];
            index += 1;
            vec.push(match cur {
                0x32 => ALOAD(Reference),
                0x53 => ASTORE(Reference),
                0x01 => ACONST_NULL,
                0x19 => LOAD(Reference, next(&mut index, bytes)? as u16),
                0x2a => LOAD(Reference, 0),
                0x2b => LOAD(Reference, 1),
                0x2c => LOAD(Reference, 2),
                0x2d => LOAD(Reference, 3),
                0xbd => ANEWARRAY(next_u16(&mut index, bytes)?),
                0xb0 => RETURN(Some(Reference)),
                0xbe => ARRAYLENGTH,
                0x3a => STORE(Reference, next(&mut index, bytes)? as u16),
                0x4b => STORE(Reference, 0),
                0x4c => STORE(Reference, 1),
                0x4d => STORE(Reference, 2),
                0x4e => STORE(Reference, 3),
                0xbf => ATHROW,
                0x33 => ALOAD(Byte),
                0x54 => ASTORE(Byte),
                0x10 => BIPUSH(next(&mut index, bytes)? as i8),
                0x34 => ALOAD(Char),
                0x55 => ASTORE(Char),
                0xc0 => CHECKCAST(next_u16(&mut index, bytes)?),
                0x90 => CONVERT(Double, Float),
                0x8e => CONVERT(Double, Int),
                0x8f => CONVERT(Double, Long),
                0x63 => ADD(Double),
                0x31 => ALOAD(Double),
                0x52 => ASTORE(Double),
                0x98 => DCMPG,
                0x97 => DCMPL,
                0x0e => DCONST_0,
                0x0f => DCONST_1,
                0x6f => DIV(Double),
                0x18 => LOAD(Double, next(&mut index, bytes)? as u16),
                0x26 => LOAD(Double, 0),
                0x27 => LOAD(Double, 1),
                0x28 => LOAD(Double, 2),
                0x29 => LOAD(Double, 3),
                0x6b => MUL(Double),
                0x77 => NEG(Double),
                0x73 => REM(Double),
                0xaf => RETURN(Some(Double)),
                0x39 => STORE(Double, next(&mut index, bytes)? as u16),
                0x47 => STORE(Double, 0),
                0x48 => STORE(Double, 1),
                0x49 => STORE(Double, 2),
                0x4a => STORE(Double, 3),
                0x67 => SUB(Double),
                0x59 => DUP,
                0x5a => DUP_X1,
                0x5b => DUP_X2,
                0x5c => DUP2,
                0x5d => DUP2_X1,
                0x5e => DUP2_X2,
                0x8d => CONVERT(Float, Double),
                0x8b => CONVERT(Float, Int),
                0x8c => CONVERT(Float, Long),
                0x62 => ADD(Float),
                0x30 => ALOAD(Float),
                0x51 => ASTORE(Float),
                0x96 => FCMPG,
                0x95 => FCMPL,
                0x0b => FCONST_0,
                0x0c => FCONST_1,
                0x0d => FCONST_2,
                0x6e => DIV(Float),
                0x17 => LOAD(Float, next(&mut index, bytes)? as u16),
                0x22 => LOAD(Float, 0),
                0x23 => LOAD(Float, 1),
                0x24 => LOAD(Float, 2),
                0x25 => LOAD(Float, 3),
                0x6a => MUL(Float),
                0x76 => NEG(Float),
                0x72 => REM(Float),
                0xae => RETURN(Some(Float)),
                0x38 => STORE(Float, next(&mut index, bytes)? as u16),
                0x43 => STORE(Float, 0),
                0x44 => STORE(Float, 1),
                0x45 => STORE(Float, 2),
                0x46 => STORE(Float, 3),
                0x66 => SUB(Float),
                0xb4 => GETFIELD(next_u16(&mut index, bytes)?),
                0xb2 => GETSTATIC(next_u16(&mut index, bytes)?),
                0xa7 => GOTO((next_u16(&mut index, bytes)? as i16) as i32),
                0xc8 => GOTO(next_u32(&mut index, bytes)? as i32),
                0x91 => CONVERT(Int, Byte),
                0x92 => CONVERT(Int, Char),
                0x87 => CONVERT(Int, Double),
                0x86 => CONVERT(Int, Float),
                0x85 => CONVERT(Int, Long),
                0x93 => CONVERT(Int, Short),
                0x60 => ADD(Int),
                0x2e => ALOAD(Int),
                0x7e => AND(Int),
                0x4f => ASTORE(Int),
                0x03 => BIPUSH(0),
                0x02 => BIPUSH(-1),
                0x04 => BIPUSH(1),
                0x05 => BIPUSH(2),
                0x06 => BIPUSH(3),
                0x07 => BIPUSH(4),
                0x08 => BIPUSH(5),
                0x6c => DIV(Int),
                0xa5 => IF_ACMP(true, next_u16(&mut index, bytes)? as i16),
                0xa6 => IF_ACMP(false, next_u16(&mut index, bytes)? as i16),
                0x9f => IF_ICMP(EQ, next_u16(&mut index, bytes)? as i16),
                0xa2 => IF_ICMP(GE, next_u16(&mut index, bytes)? as i16),
                0xa3 => IF_ICMP(GT, next_u16(&mut index, bytes)? as i16),
                0xa4 => IF_ICMP(LE, next_u16(&mut index, bytes)? as i16),
                0xa1 => IF_ICMP(LT, next_u16(&mut index, bytes)? as i16),
                0xa0 => IF_ICMP(NE, next_u16(&mut index, bytes)? as i16),
                0x99 => IF(EQ, next_u16(&mut index, bytes)? as i16),
                0x9c => IF(GE, next_u16(&mut index, bytes)? as i16),
                0x9d => IF(GT, next_u16(&mut index, bytes)? as i16),
                0x9e => IF(LE, next_u16(&mut index, bytes)? as i16),
                0x9b => IF(LT, next_u16(&mut index, bytes)? as i16),
                0x9a => IF(NE, next_u16(&mut index, bytes)? as i16),
                0xc7 => IFNULL(false, next_u16(&mut index, bytes)? as i16),
                0xc6 => IFNULL(true, next_u16(&mut index, bytes)? as i16),
                0x84 => {
                    IINC(next(&mut index, bytes)? as u16,
                         (next(&mut index, bytes)? as i8) as i16)
                }
                0x15 => LOAD(Int, next(&mut index, bytes)? as u16),
                0x1a => LOAD(Int, 0),
                0x1b => LOAD(Int, 1),
                0x1c => LOAD(Int, 2),
                0x1d => LOAD(Int, 3),
                0x68 => MUL(Int),
                0x74 => NEG(Int),
                0xc1 => INSTANCEOF(next_u16(&mut index, bytes)?),
                0xb9 => {
                    let op = INVOKEINTERFACE(next_u16(&mut index, bytes)?,
                                             next(&mut index, bytes)?);
                    next(&mut index, bytes)?; // discard 0
                    op
                }
                0xb7 => INVOKESPECIAL(next_u16(&mut index, bytes)?),
                0xb8 => INVOKESTATIC(next_u16(&mut index, bytes)?),
                0xb6 => INVOKEVIRTUAL(next_u16(&mut index, bytes)?),
                0x80 => OR(Int),
                0x70 => REM(Int),
                0xac => RETURN(Some(Int)),
                0x78 => SHL(Int),
                0x7a => SHR(Int),
                0x36 => STORE(Int, next(&mut index, bytes)? as u16),
                0x3b => STORE(Int, 0),
                0x3c => STORE(Int, 1),
                0x3d => STORE(Int, 2),
                0x3e => STORE(Int, 3),
                0x64 => SUB(Int),
                0x7c => USHR(Int),
                0x82 => XOR(Int),
                0xa8 => JSR((next_u16(&mut index, bytes)? as i16) as i32),
                0xc9 => JSR(next_u32(&mut index, bytes)? as i32),
                0x8a => CONVERT(Long, Double),
                0x89 => CONVERT(Long, Float),
                0x88 => CONVERT(Long, Int),
                0x61 => ADD(Long),
                0x2f => ALOAD(Long),
                0x7f => AND(Long),
                0x50 => ASTORE(Long),
                0x94 => LCMP,
                0x09 => LCONST_0,
                0x0a => LCONST_1,
                0x12 => LDC(next(&mut index, bytes)? as u16),
                0x13 => LDC(next_u16(&mut index, bytes)?),
                0x14 => LDC2_W(next_u16(&mut index, bytes)?),
                0x6d => DIV(Long),
                0x16 => LOAD(Long, next(&mut index, bytes)? as u16),
                0x1e => LOAD(Long, 0),
                0x1f => LOAD(Long, 1),
                0x20 => LOAD(Long, 2),
                0x21 => LOAD(Long, 3),
                0x69 => MUL(Long),
                0x75 => NEG(Long),
                // 0xab => lookupswitch, // TODO
                0x81 => OR(Long),
                0x71 => REM(Long),
                0xad => RETURN(Some(Long)),
                0x79 => SHL(Long),
                0x7b => SHR(Long),
                0x37 => STORE(Long, next(&mut index, bytes)? as u16),
                0x3f => STORE(Long, 0),
                0x40 => STORE(Long, 1),
                0x41 => STORE(Long, 2),
                0x42 => STORE(Long, 3),
                0x65 => SUB(Long),
                0x7d => USHR(Long),
                0x83 => XOR(Long),
                0xc2 => MONITORENTER,
                0xc3 => MONITOREXIT,
                0xc5 => MULTIANEWARRAY(next_u16(&mut index, bytes)?, next(&mut index, bytes)?),
                0xbb => NEW(next_u16(&mut index, bytes)?),
                0xbc => {
                    NEWARRAY(match next(&mut index, bytes)? {
                        4 => Boolean,
                        5 => Char,
                        6 => Float,
                        7 => Double,
                        8 => Byte,
                        9 => Short,
                        10 => Int,
                        11 => Long,
                        c @ _ => return Err(format!("unknown array type: {}", c)),
                    })
                }
                0x00 => NOP,
                0x57 => POP,
                0x58 => POP2,
                0xb5 => PUTFIELD(next_u16(&mut index, bytes)?),
                0xb3 => PUTSTATIC(next_u16(&mut index, bytes)?),
                0xa9 => RET(next(&mut index, bytes)? as u16),
                0xb1 => RETURN(None),
                0x35 => ALOAD(Short),
                0x56 => ASTORE(Short),
                0x11 => SIPUSH(next_u16(&mut index, bytes)? as i16),
                0x5f => SWAP,
                // 0xaa => tableswitch, // TODO
                // 0xc4 => wide, // TODO
                op @ _ => return Err(format!("Unknown Instruction {:#x}", op)),
            });
        }

        // TODO: gotos/if anpassen
        return Ok(vec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Instruction::*;
    use super::Type::*;
    use class::Class;
    use classfile_parser::parse_class;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn get_instructions(method_name: &str) -> Vec<Instruction> {
        Class::from_class_file(&parse_class("./assets/TestInstruction").unwrap()).unwrap()
            .method_by_name(method_name).unwrap()
            .code().unwrap()
            .code().to_vec()
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_conversions() {
        assert_eq!(get_instructions("conversions"),
                   vec![BIPUSH(1), STORE(Int, 1),
                        LOAD(Int, 1),    CONVERT(Int, Byte), STORE(Int, 2),
                        LOAD(Int, 1),    CONVERT(Int, Short), STORE(Int, 3),
                        LOAD(Int, 1),    CONVERT(Int, Long), STORE(Long, 4),
                        LOAD(Int, 1),    CONVERT(Int, Float), STORE(Float, 6),
                        LOAD(Int, 1),    CONVERT(Int, Double), STORE(Double, 7),
                        LOAD(Long, 4),   CONVERT(Long, Int), STORE(Int, 1),
                        LOAD(Long, 4),   CONVERT(Long, Float), STORE(Float, 6),
                        LOAD(Long, 4),   CONVERT(Long, Double), STORE(Double, 7),
                        LOAD(Float, 6),  CONVERT(Float, Int), STORE(Int, 1),
                        LOAD(Float, 6),  CONVERT(Float, Long), STORE(Long, 4),
                        LOAD(Float, 6),  CONVERT(Float, Double), STORE(Double, 7),
                        LOAD(Double, 7), CONVERT(Double, Int), STORE(Int, 1),
                        LOAD(Double, 7), CONVERT(Double, Long), STORE(Long, 4),
                        LOAD(Double, 7), CONVERT(Double, Float), STORE(Float, 6),
                        RETURN(None)]);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_arithmetic() {
        assert_eq!(get_instructions("arithmetic"),
                   vec![BIPUSH(1), STORE(Int, 1),
                        LCONST_1, STORE(Long, 2),
                        FCONST_1, STORE(Float, 4),
                        DCONST_1, STORE(Double, 5),
                        LOAD(Int, 1), BIPUSH(1), ADD(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), SUB(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), MUL(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), DIV(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), REM(Int), STORE(Int, 1),
                        LOAD(Int, 1), NEG(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), SHL(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), SHR(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), USHR(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), AND(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), OR(Int), STORE(Int, 1),
                        LOAD(Int, 1), BIPUSH(1), XOR(Int), STORE(Int, 1),
                        IINC(1, -10),
                        LOAD(Long, 2), LCONST_1, ADD(Long), STORE(Long, 2),
                        LOAD(Long, 2), LCONST_1, SUB(Long), STORE(Long, 2),
                        LOAD(Long, 2), LCONST_1, MUL(Long), STORE(Long, 2),
                        LOAD(Long, 2), LCONST_1, DIV(Long), STORE(Long, 2),
                        LOAD(Long, 2), LCONST_1, REM(Long), STORE(Long, 2),
                        LOAD(Long, 2), NEG(Long), STORE(Long, 2),
                        LOAD(Long, 2), BIPUSH(1), SHL(Long), STORE(Long, 2),
                        LOAD(Long, 2), BIPUSH(1), SHR(Long), STORE(Long, 2),
                        LOAD(Long, 2), BIPUSH(1), USHR(Long), STORE(Long, 2),
                        LOAD(Long, 2), LCONST_1, AND(Long), STORE(Long, 2),
                        LOAD(Long, 2), LCONST_1, OR(Long), STORE(Long, 2),
                        LOAD(Long, 2), LCONST_1, XOR(Long), STORE(Long, 2),
                        LOAD(Float, 4), FCONST_1, ADD(Float), STORE(Float, 4),
                        LOAD(Float, 4), FCONST_1, SUB(Float), STORE(Float, 4),
                        LOAD(Float, 4), FCONST_1, MUL(Float), STORE(Float, 4),
                        LOAD(Float, 4), FCONST_1, DIV(Float), STORE(Float, 4),
                        LOAD(Float, 4), FCONST_1, REM(Float), STORE(Float, 4),
                        LOAD(Float, 4), NEG(Float), STORE(Float, 4),
                        LOAD(Double, 5), DCONST_1, ADD(Double), STORE(Double, 5),
                        LOAD(Double, 5), DCONST_1, SUB(Double), STORE(Double, 5),
                        LOAD(Double, 5), DCONST_1, MUL(Double), STORE(Double, 5),
                        LOAD(Double, 5), DCONST_1, DIV(Double), STORE(Double, 5),
                        LOAD(Double, 5), DCONST_1, REM(Double), STORE(Double, 5),
                        LOAD(Double, 5), NEG(Double), STORE(Double, 5),
                        RETURN(None)]);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_reference() {
        assert_eq!(get_instructions("reference"),
                   vec![ACONST_NULL, STORE(Reference, 1),
                        LOAD(Reference, 1), STORE(Reference, 2),
                        RETURN(None)]);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_array() {
        assert_eq!(get_instructions("array"),
                   vec![BIPUSH(2), NEWARRAY(Boolean), STORE(Reference, 1),
                        LOAD(Reference, 1), BIPUSH(0), LOAD(Reference, 1), BIPUSH(1), ALOAD(Byte), ASTORE(Byte),
                        BIPUSH(2), NEWARRAY(Byte), STORE(Reference, 2),
                        LOAD(Reference, 2), BIPUSH(0), LOAD(Reference, 2), BIPUSH(1), ALOAD(Byte), ASTORE(Byte),
                        BIPUSH(2), NEWARRAY(Short), STORE(Reference, 3),
                        LOAD(Reference, 3), BIPUSH(0), LOAD(Reference, 3), BIPUSH(1), ALOAD(Short), ASTORE(Short),
                        BIPUSH(2), NEWARRAY(Int), STORE(Reference, 4),
                        LOAD(Reference, 4), BIPUSH(0), LOAD(Reference, 4), BIPUSH(1), ALOAD(Int), ASTORE(Int),
                        BIPUSH(2), NEWARRAY(Long), STORE(Reference, 5),
                        LOAD(Reference, 5), BIPUSH(0), LOAD(Reference, 5), BIPUSH(1), ALOAD(Long), ASTORE(Long),
                        BIPUSH(2), NEWARRAY(Float), STORE(Reference, 6),
                        LOAD(Reference, 6), BIPUSH(0), LOAD(Reference, 6), BIPUSH(1), ALOAD(Float), ASTORE(Float),
                        BIPUSH(2), NEWARRAY(Double), STORE(Reference, 7),
                        LOAD(Reference, 7), BIPUSH(0), LOAD(Reference, 7), BIPUSH(1), ALOAD(Double), ASTORE(Double),
                        BIPUSH(2), NEWARRAY(Char), STORE(Reference, 8),
                        LOAD(Reference, 8), BIPUSH(0), LOAD(Reference, 8), BIPUSH(1), ALOAD(Char), ASTORE(Char),
                        BIPUSH(2), ANEWARRAY(2), STORE(Reference, 9), LOAD(Reference, 9),
                        BIPUSH(0), LOAD(Reference, 9), BIPUSH(1), ALOAD(Reference), ASTORE(Reference),
                        BIPUSH(2), BIPUSH(2), MULTIANEWARRAY(3, 2), STORE(Reference, 10),
                        LOAD(Reference, 10), BIPUSH(0), ALOAD(Reference), BIPUSH(0), LOAD(Reference, 10), BIPUSH(1), ALOAD(Reference), BIPUSH(1), ALOAD(Reference), ASTORE(Reference),
                        RETURN(None)]);
    }

}
