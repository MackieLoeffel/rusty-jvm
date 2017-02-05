#[derive(Debug)]
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

#[derive(Debug)]
pub enum Type {
    Reference,
    Char,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
}

#[derive(Debug)]
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
                0x6e => fdiv,
                0x17 => fload,
                0x22 => fload_0,
                0x23 => fload_1,
                0x24 => fload_2,
                0x25 => fload_3,
                // 0x6a => fmul,
                // 0x76 => fneg,
                // 0x72 => frem,
                // 0xae => freturn,
                // 0x38 => fstore,
                // 0x43 => fstore_0,
                // 0x44 => fstore_1,
                // 0x45 => fstore_2,
                // 0x46 => fstore_3,
                // 0x66 => fsub,
                // 0xb4 => getfield,
                // 0xb2 => getstatic,
                // 0xa7 => goto,
                // 0xc8 => goto_w,
                // 0x91 => i2b,
                // 0x92 => i2c,
                // 0x87 => i2d,
                // 0x86 => i2f,
                // 0x85 => i2l,
                // 0x93 => i2s,
                // 0x60 => iadd,
                // 0x2e => iaload,
                // 0x7e => iand,
                // 0x4f => iastore,
                // 0x02 => iconst_m1,
                // 0x03 => iconst_0,
                // 0x04 => iconst_1,
                // 0x05 => iconst_2,
                // 0x06 => iconst_3,
                // 0x07 => iconst_4,
                // 0x08 => iconst_5,
                // 0x6c => idiv,
                // 0xa5 => if_acmpeq,
                // 0xa6 => if_acmpne,
                // 0x9f => if_icmpeq,
                // 0xa2 => if_icmpge,
                // 0xa3 => if_icmpgt,
                // 0xa4 => if_icmple,
                // 0xa1 => if_icmplt,
                // 0xa0 => if_icmpne,
                // 0x99 => ifeq,
                // 0x9c => ifge,
                // 0x9d => ifgt,
                // 0x9e => ifle,
                // 0x9b => iflt,
                // 0x9a => ifne,
                // 0xc7 => ifnonnull,
                // 0xc6 => ifnull,
                // 0x84 => iinc,
                // 0x15 => iload,
                // 0x1a => iload_0,
                // 0x1b => iload_1,
                // 0x1c => iload_2,
                // 0x1d => iload_3,
                // 0xfe => impdep1,
                // 0xff => impdep2,
                // 0x68 => imul,
                // 0x74 => ineg,
                // 0xc1 => instanceof,
                // 0xba => invokedynamic,
                // 0xb9 => invokeinterface,
                // 0xb7 => invokespecial,
                // 0xb8 => invokestatic,
                // 0xb6 => invokevirtual,
                // 0x80 => ior,
                // 0x70 => irem,
                // 0xac => ireturn,
                // 0x78 => ishl,
                // 0x7a => ishr,
                // 0x36 => istore,
                // 0x3b => istore_0,
                // 0x3c => istore_1,
                // 0x3d => istore_2,
                // 0x3e => istore_3,
                // 0x64 => isub,
                // 0x7c => iushr,
                // 0x82 => ixor,
                // 0xa8 => jsr,
                // 0xc9 => jsr_w,
                // 0x8a => l2d,
                // 0x89 => l2f,
                // 0x88 => l2i,
                // 0x61 => ladd,
                // 0x2f => laload,
                // 0x7f => land,
                // 0x50 => lastore,
                // 0x94 => lcmp,
                // 0x09 => lconst_0,
                // 0x0a => lconst_1,
                // 0x12 => ldc,
                // 0x13 => ldc_w,
                // 0x14 => ldc2_w,
                // 0x6d => ldiv,
                // 0x16 => lload,
                // 0x1e => lload_0,
                // 0x1f => lload_1,
                // 0x20 => lload_2,
                // 0x21 => lload_3,
                // 0x69 => lmul,
                // 0x75 => lneg,
                // 0xab => lookupswitch,
                // 0x81 => lor,
                // 0x71 => lrem,
                // 0xad => lreturn,
                // 0x79 => lshl,
                // 0x7b => lshr,
                // 0x37 => lstore,
                // 0x3f => lstore_0,
                // 0x40 => lstore_1,
                // 0x41 => lstore_2,
                // 0x42 => lstore_3,
                // 0x65 => lsub,
                // 0x7d => lushr,
                // 0x83 => lxor,
                // 0xc2 => monitorenter,
                // 0xc3 => monitorexit,
                // 0xc5 => multianewarray,
                // 0xbb => new,
                // 0xbc => newarray,
                // 0x00 => nop,
                // 0x57 => pop,
                // 0x58 => pop2,
                // 0xb5 => putfield,
                // 0xb3 => putstatic,
                // 0xa9 => ret,
                // 0xb1 => return,
                // 0x35 => saload,
                // 0x56 => sastore,
                // 0x11 => sipush,
                // 0x5f => swap,
                // 0xaa => tableswitch,
                // 0xc4 => wide,
                op@_ => return Err(format!("Unknown Instruction {:#x}", op)),
            });
        }

        // TODO: gotos/if anpassen
        return Ok(vec);
    }
}
