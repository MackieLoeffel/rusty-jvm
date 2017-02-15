use classfile_parser::method_info::{PUBLIC, STATIC, NATIVE};
use class_loader::ClassLoader;
use instruction::{Instruction, LocalVarRef};
use instruction::Instruction::*;
use instruction::Type::*;
use parsed_class::MethodRef;
use descriptor::{FieldDescriptor, MethodDescriptor};
use object::{Object, ArrayObject, InstanceObject};
use class::Class;
use std::mem;
use std::char;
use std::ops::{Mul, Add, Div, Sub, Rem, BitAnd, BitOr, BitXor};

// USE WITH CARE
macro_rules! conv { ($val: expr) => {{unsafe {mem::transmute($val)}}} }

pub struct VM {
    classloader: ClassLoader,
    frames: Vec<Frame>,
    heap: Vec<Option<Object>>,
    // TODO #[cfg(debug)]
    native_calls: Vec<(String, String, Vec<i32>)>,
}

pub struct Frame {
    // arrays are to big to create them on the stack
    // TODO think about using maybe Box<Frame> with arrays
    //  => benchmark
    code: Vec<Instruction>,
    ip: usize,
    sp: usize,
    local_vars: Vec<i32>,
    stack: Vec<i32>,
    current_class: String,
}

impl VM {
    pub fn new(loader: ClassLoader) -> VM {
        let mut heap = Vec::new();
        // create dummy null object
        heap.push(None);

        VM {
            native_calls: Vec::new(),
            classloader: loader,
            frames: Vec::new(),
            heap: heap,
        }
    }

    // TODO think about using a real error type here
    pub fn start(&mut self, class: &str, _args: &[&str]) -> Result<(), String> {
        let class_name;
        {
            let start_class = self.classloader.load_class(class).map_err(|err| format!("ClassLoadingError: {}", err))?;

            let main = match start_class.method_by_signature("main", "([Ljava/lang/String;)V") {
                Some(m) => m,
                None => return Err("No main method found!".to_owned()),
            };

            if main.access_flags() != PUBLIC | STATIC {
                return Err(format!("invalid access flags for main: {:?}", main.access_flags()));
            }

            class_name = start_class.name().to_owned();
        }
        let mut start_frame = Frame::dummy_frame(1);
        start_frame.push(0); // TODO push real args on the stack

        self.invoke_method(&class_name,
                           "main",
                           "([Ljava/lang/String;)V",
                           &mut start_frame);

        self.run(start_frame);
        Ok(())
    }

    fn invoke_method(&mut self, class_name: &str, method: &str, descriptor: &str, calling_frame: &mut Frame) {
        // these unwraps should be checked in the linking stage
        let method = self.classloader.load_class(class_name).unwrap().method_by_signature(method, descriptor).unwrap();

        let mut local_vars;
        let code;
        {
            let args = &calling_frame.stack[calling_frame.sp - method.words_for_params()..calling_frame.sp];
            calling_frame.sp -= method.words_for_params();

            if method.access_flags().contains(NATIVE) {
                self.native_calls.push((method.name().to_owned(), method.descriptor().to_owned(), args.to_vec()));
                // TODO real handling of call
                return;
            }
            // TODO remove specialhandling for dump_char, when there is another way to output
            if method.name() == "dump_char" && method.descriptor() == "(C)V" {
                print!("{}", char::from_u32(args[0] as u32).unwrap_or('?'));
                return;
            }

            code = method.code().expect("Method must have code");
            // println!("Code: {:?}", code);

            local_vars = Vec::with_capacity(code.max_locals());
            local_vars.resize(code.max_locals(), 0);
            local_vars[..args.len()].copy_from_slice(args);
        }
        let mut stack = Vec::with_capacity(code.max_stack());
        stack.resize(code.max_stack(), 0);

        let mut new_frame = Frame {
            ip: 0,
            sp: 0,
            local_vars: local_vars,
            stack: stack,
            code: code.code().clone(),
            current_class: class_name.to_owned(),
        };
        mem::swap(&mut new_frame, calling_frame);
        self.frames.push(new_frame);
    }

    fn invoke_method_ref(&mut self, method: &MethodRef, calling_frame: &mut Frame) {
        self.invoke_method(method.class(),
                           method.name(),
                           method.descriptor(),
                           calling_frame)
    }

    fn allocate_object(&mut self, object: Object) -> i32 {
        // TODO think of a better allocation scheeme

        // println!("Allocating object {:?}", object);
        for i in 1..self.heap.len() {
            if self.heap[i].is_none() {
                self.heap[i] = Some(object);
                return i as i32;
            }
        }
        self.heap.push(Some(object));
        (self.heap.len() - 1) as i32
    }
    fn get_object(heap: &mut Vec<Option<Object>>, index: i32) -> &mut Object {

        heap[index as usize].as_mut().expect("Invalid Reference")
    }

    fn get_array(&mut self, index: i32) -> &mut ArrayObject { VM::get_object(&mut self.heap, index).as_array() }

    fn get_instance(heap: &mut Vec<Option<Object>>, index: i32) -> &mut InstanceObject {
        VM::get_object(heap, index).as_instance()
    }

    fn run(&mut self, start_frame: Frame) {
        self.frames.pop().expect("Expected dummy frame on frame stack");
        assert_eq!(self.frames.len(), 0);
        let mut frame = start_frame;

        macro_rules! arith_int(($typ: ident, $op:ident) => {{
            match $typ {
                Int => {
                    let b: i32 = conv!(frame.pop());
                    let a: i32 = conv!(frame.pop());
                    frame.push(conv!(a.$op(b)));
                }
                Long => {
                    let b: i64 = conv!(frame.pop2());
                    let a: i64 = conv!(frame.pop2());
                    frame.push2(conv!(a.$op(b)));
                }
                t => panic!("Operation {} is not implemented for typ {:?}", stringify!($op), t),
            }
        }});

        macro_rules! arith_float(($typ: ident, $op:ident) => {{
            match $typ {
                Float => {
                    let b: f32 = conv!(frame.pop());
                    let a: f32 = conv!(frame.pop());
                    frame.push(conv!(a.$op(b)));
                }
                Double => {
                    let b: f64 = conv!(frame.pop2());
                    let a: f64 = conv!(frame.pop2());
                    frame.push2(conv!(a.$op(b)));
                }
                t => panic!("Operation {} is not implemented for typ {:?}", stringify!($op), t),
            }
        }});
        macro_rules! convert(($from_typ: ident, $pop: ident, $to_typ: ident, $push: ident) => {{
            let a: $from_typ = conv!(frame.$pop());
            frame.$push(conv!(a as $to_typ));
        }});


        loop {
            match frame.next_instruction() {
                ASTORE(typ) => {
                    if typ.is_double_sized() {
                        let val = frame.pop2();
                        let index = frame.pop();
                        let array = frame.pop();
                        // TODO throw nullpointer exception

                        self.get_array(array).set2(index, val);
                    } else {
                        let val = frame.pop();
                        let index = frame.pop();
                        let array = frame.pop();
                        // TODO throw nullpointer exception

                        self.get_array(array).set(index, val);
                    }
                }
                ALOAD(typ) => {
                    let index = frame.pop();
                    let array = frame.pop();
                    // TODO throw nullpointer exception
                    if typ.is_double_sized() {
                        frame.push2(self.get_array(array).get2(index));
                    } else {
                        frame.push(self.get_array(array).get(index));
                    }
                }
                ARRAYLENGTH => {
                    let array = frame.pop();
                    frame.push(self.get_array(array).length());
                }

                STORE(typ, idx) => {
                    if typ.is_double_sized() {
                        // TODO test
                        let v = frame.pop2();
                        frame.store2(idx, v);
                    } else {
                        let v = frame.pop();
                        frame.store(idx, v);
                    }
                }
                LOAD(typ, idx) => {
                    if typ.is_double_sized() {
                        let v = frame.load2(idx);
                        frame.push2(v);
                    } else {
                        let v = frame.load(idx);
                        frame.push(v);
                    }
                }

                ANEWARRAY(_) => {
                    let length = frame.pop();
                    // TODO throw NegativeArraySizeException exception
                    frame.push(self.allocate_object(Object::new_array(length, Reference)));
                }
                MULTIANEWARRAY(descriptor, count) => {
                    fn create_array(depth: usize,
                                    count: usize,
                                    desc: &FieldDescriptor,
                                    frame: &mut Frame,
                                    vm: &mut VM)
                                    -> i32 {
                        let len = frame.nth_from_top(count - depth);
                        // TODO throw NegativeArraySizeException exception
                        let typ = if depth == count {
                            desc.as_type_without_arrays(count)
                        } else {
                            Reference
                        };
                        let mut array = ArrayObject::new(len, typ);

                        if depth < count {
                            for i in 0..len {
                                array.set(i, create_array(depth + 1, count, desc, frame, vm));
                            }
                        }
                        vm.allocate_object(Object::Array(array))
                    }

                    let created = create_array(1,
                                               count as usize,
                                               &FieldDescriptor::parse(&descriptor).unwrap(),
                                               &mut frame,
                                               self);
                    frame.sp -= count as usize;
                    frame.push(created);
                }
                NEW(class) => {
                    let instance = match Object::new_instance(&class, &mut self.classloader) {
                        Ok(i) => i,
                        // TODO throw class laoding exception
                        Err(e) => panic!("Error loading class {}: {}", class, e),
                    };
                    frame.push(self.allocate_object(instance));
                }
                NEWARRAY(t) => {
                    let length = frame.pop();
                    // TODO throw NegativeArraySizeException exception
                    frame.push(self.allocate_object(Object::new_array(length, t)));
                }

                CONVERT(Int, Byte) => {
                    let a = frame.pop();
                    frame.push(a as i8 as i32);
                }
                CONVERT(Int, Short) => {
                    let a = frame.pop();
                    frame.push(a as i16 as i32);
                }
                CONVERT(Int, Char) => {
                    let a = frame.pop();
                    frame.push(a as u16 as i32);
                }

                CONVERT(Int, Long) => convert!(i32, pop, i64, push2),
                CONVERT(Int, Float) => convert!(i32, pop, f32, push),
                CONVERT(Int, Double) => convert!(i32, pop, f64, push2),
                CONVERT(Long, Int) => convert!(i64, pop2, i32, push),
                CONVERT(Long, Float) => convert!(i64, pop2, f32, push),
                CONVERT(Long, Double) => convert!(i64, pop2, f64, push2),
                CONVERT(Float, Int) => convert!(f32, pop, i32, push),
                CONVERT(Float, Long) => convert!(f32, pop, i64, push2),
                CONVERT(Float, Double) => convert!(f32, pop, f64, push2),
                CONVERT(Double, Int) => convert!(f64, pop2, i32, push),
                CONVERT(Double, Long) => convert!(f64, pop2, i64, push2),
                CONVERT(Double, Float) => convert!(f64, pop2, f32, push),

                ADD(t @ Int) | ADD(t @ Long) => arith_int!(t, wrapping_add),
                ADD(t) => arith_float!(t, add),
                SUB(t @ Int) | SUB(t @ Long) => arith_int!(t, wrapping_sub),
                SUB(t) => arith_float!(t, sub),
                MUL(t @ Int) | MUL(t @ Long) => arith_int!(t, wrapping_mul),
                MUL(t) => arith_float!(t, mul),
                // TODO arithmetic exception
                DIV(t @ Int) | DIV(t @ Long) => arith_int!(t, wrapping_div),
                DIV(t) => arith_float!(t, div),
                REM(t @ Int) | REM(t @ Long) => arith_int!(t, wrapping_rem),
                REM(t) => arith_float!(t, rem),
                AND(t) => arith_int!(t, bitand),
                OR(t) => arith_int!(t, bitor),
                XOR(t) => arith_int!(t, bitxor),
                NEG(t) => {
                    match t {
                        Int => {
                            let a: i32 = conv!(frame.pop());
                            frame.push(conv!(a.wrapping_neg()));
                        }
                        Long => {
                            let a: i64 = conv!(frame.pop2());
                            frame.push2(conv!(a.wrapping_neg()));
                        }
                        Float => {
                            let a: f32 = conv!(frame.pop());
                            frame.push(conv!(-a));
                        }
                        Double => {
                            let a: f64 = conv!(frame.pop2());
                            frame.push2(conv!(-a));
                        }
                        t => panic!("Operation NEG is not implemented for typ {:?}", t),
                    }
                }
                SHL(t) => {
                    match t {
                        Int => {
                            let b: u32 = conv!(frame.pop());
                            let a: i32 = conv!(frame.pop());
                            frame.push(conv!(a.wrapping_shl(b)));
                        }
                        Long => {
                            let b: u32 = conv!(frame.pop());
                            let a: i64 = conv!(frame.pop2());
                            frame.push2(conv!(a.wrapping_shl(b)));
                        }
                        t => panic!("Operation SHL is not implemented for typ {:?}", t),
                    }
                }
                SHR(t) => {
                    match t {
                        Int => {
                            let b: u32 = conv!(frame.pop());
                            let a: i32 = conv!(frame.pop());
                            frame.push(conv!(a.wrapping_shr(b)));
                        }
                        Long => {
                            let b: u32 = conv!(frame.pop());
                            let a: i64 = conv!(frame.pop2());
                            frame.push2(conv!(a.wrapping_shr(b)));
                        }
                        t => panic!("Operation SHR is not implemented for typ {:?}", t),
                    }
                }
                USHR(t) => {
                    match t {
                        Int => {
                            let b: u32 = conv!(frame.pop());
                            let a: u32 = conv!(frame.pop());
                            frame.push(conv!(a.wrapping_shr(b)));
                        }
                        Long => {
                            let b: u32 = conv!(frame.pop());
                            let a: u64 = conv!(frame.pop2());
                            frame.push2(conv!(a.wrapping_shr(b)));
                        }
                        t => panic!("Operation USHR is not implemented for typ {:?}", t),
                    }
                }
                RETURN(o) => {
                    if self.frames.is_empty() {
                        return;
                    }
                    let mut old_frame = frame;
                    frame = self.frames.pop().unwrap();

                    if let Some(typ) = o {
                        if typ.is_double_sized() {
                            // TODO test
                            let v2 = old_frame.pop2();
                            frame.push2(v2);
                        } else {
                            let v = old_frame.pop();
                            frame.push(v);
                        }
                    }
                }
                IINC(var, val) => {
                    let a = frame.load(var);
                    frame.store(var, a.wrapping_add(val as i32));
                }

                ACONST_NULL => frame.push(0),
                DCONST_0 => frame.push2(conv!(0f64)),
                DCONST_1 => frame.push2(conv!(1f64)),
                FCONST_0 => frame.push(conv!(0f32)),
                FCONST_1 => frame.push(conv!(1f32)),
                FCONST_2 => frame.push(conv!(2f32)),
                LCONST_0 => frame.push2(conv!(0i64)),
                LCONST_1 => frame.push2(conv!(1i64)),
                BIPUSH(i) => frame.push(i as i32),
                SIPUSH(i) => frame.push(i as i32),
                LDC_INT(i) => frame.push(i),
                LDC_FLOAT(f) => frame.push(conv!(f)),
                // TODO LDC_STRING(String) => frame.push(),
                LDC_DOUBLE(f) => frame.push2(conv!(f)),
                LDC_LONG(i) => frame.push2(conv!(i)),

                DUP => {
                    let val = frame.top();
                    frame.push(val);
                }
                POP => {
                    frame.pop();
                }

                GETFIELD(field) => {
                    let objindex = frame.pop();
                    // TODO throw NullPointerException
                    let obj = VM::get_instance(&mut self.heap, objindex);
                    if field.typ().is_double_sized() {
                        // TODO replace unwrap with exception throw
                        frame.push2(obj.get_field2(&field, &mut self.classloader).unwrap());
                    } else {
                        // TODO replace unwrap with exception throw
                        frame.push(obj.get_field(&field, &mut self.classloader).unwrap());
                    }
                }
                PUTFIELD(field) => {
                    if field.typ().is_double_sized() {
                        let value = frame.pop2();
                        let objindex = frame.pop();
                        // TODO throw NullPointerException
                        let obj = VM::get_instance(&mut self.heap, objindex);
                        // TODO replace unwrap with exception throw
                        obj.set_field2(&field, value, &mut self.classloader).unwrap();
                    } else {
                        let value = frame.pop();
                        let objindex = frame.pop();
                        // TODO throw NullPointerException
                        let obj = VM::get_instance(&mut self.heap, objindex);
                        // TODO replace unwrap with exception throw
                        obj.set_field(&field, value, &mut self.classloader).unwrap();
                    }
                }

                i @ DCMPG | i @ DCMPL => {
                    let b: f64 = conv!(frame.pop2());
                    let a: f64 = conv!(frame.pop2());
                    frame.push(if a == b {
                        0
                    } else if a < b {
                        -1
                    } else if a > b {
                        1
                    } else {
                        // one is NaN
                        if i == DCMPG { 1 } else { 0 }
                    });
                }
                i @ FCMPG | i @ FCMPL => {
                    let b: f32 = conv!(frame.pop());
                    let a: f32 = conv!(frame.pop());
                    frame.push(if a == b {
                        0
                    } else if a < b {
                        -1
                    } else if a > b {
                        1
                    } else {
                        // one is NaN
                        if i == FCMPG { 1 } else { 0 }
                    });
                }
                LCMP => {
                    let b: i64 = conv!(frame.pop2());
                    let a: i64 = conv!(frame.pop2());
                    frame.push(if a == b {
                        0
                    } else if a < b {
                        -1
                    } else {
                        1
                    });
                }

                GOTO(dest) => frame.ip = dest as usize,
                IF_ACMP(equal, dest) => {
                    let b = frame.pop();
                    let a = frame.pop();
                    if (a == b) == equal {
                        frame.ip = dest as usize;
                    }
                }
                IF_ICMP(comp, dest) => {
                    let b = frame.pop();
                    let a = frame.pop();
                    if comp.compare(a, b) {
                        frame.ip = dest as usize;
                    }
                }
                IF(comp, dest) => {
                    let a = frame.pop();
                    if comp.compare(a, 0) {
                        frame.ip = dest as usize;
                    }
                }
                IFNULL(equal, dest) => {
                    let a = frame.pop();
                    if (a == 0) == equal {
                        frame.ip = dest as usize;
                    }
                }

                INVOKESPECIAL(method) => {
                    // special lookup procedure for invoke special
                    // see https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc6.html
                    // TODO replace unwraps with throw class loading exception
                    if self.classloader.load_class(&frame.current_class).unwrap().has_acc_super_flag() &&
                       method.name() != "<init>" &&
                       Class::is_real_super_class(method.class(), &frame.current_class, &mut self.classloader)
                        .unwrap() {
                        let dest_class = Class::find_first_real_super_class_with_method(&frame.current_class,
                                                                                        method.name(),
                                                                                        method.descriptor(),
                                                                                        &mut self.classloader)
                            .unwrap()
                            .unwrap();
                        self.invoke_method(&dest_class, method.name(), method.descriptor(), &mut frame);
                    } else {
                        self.invoke_method_ref(&method, &mut frame);
                    }
                }
                INVOKEVIRTUAL(method) => {
                    let dest_class;
                    {
                        let object_offset = MethodDescriptor::parse(method.descriptor()).unwrap().words_for_params();
                        // TODO throw null pointer exception
                        let object = VM::get_instance(&mut self.heap, frame.nth_from_top(object_offset));
                        dest_class = Class::find_first_super_class_with_method(object.class(),
                                                                               method.name(),
                                                                               method.descriptor(),
                                                                               &mut self.classloader)
                            .unwrap()
                            .unwrap();
                    }
                    self.invoke_method(&dest_class, method.name(), method.descriptor(), &mut frame);
                }
                INVOKESTATIC(method) => {
                    self.invoke_method_ref(&method, &mut frame);
                }
                c => panic!("Not implemented Instruction {:?}", c),
            }
        }
    }
}

impl Frame {
    fn dummy_frame(stack_size: usize) -> Frame {
        let mut stack = Vec::with_capacity(stack_size);
        stack.resize(stack_size, 0);
        Frame {
            ip: 0,
            sp: 0,
            stack: stack,
            local_vars: Vec::new(),
            code: Vec::new(),
            current_class: "".to_owned(),
        }
    }

    #[inline(always)]
    fn next_instruction(&mut self) -> Instruction {
        let instruction = self.code[self.ip].clone();
        self.ip += 1;
        instruction
    }

    #[inline(always)]
    fn push(&mut self, val: i32) {
        self.stack[self.sp] = val;
        self.sp += 1;
    }

    #[inline(always)]
    fn push2(&mut self, val: [i32; 2]) {
        self.push(val[0]);
        self.push(val[1]);
    }

    #[inline(always)]
    fn pop(&mut self) -> i32 {
        self.sp -= 1;
        self.stack[self.sp]
    }

    #[inline(always)]
    fn pop2(&mut self) -> [i32; 2] {
        let b = self.pop();
        let a = self.pop();
        [a, b]
    }

    #[inline(always)]
    fn top(&self) -> i32 { self.nth_from_top(0) }

    #[inline(always)]
    fn nth_from_top(&self, n: usize) -> i32 { self.stack[self.sp - 1 - n] }

    #[inline(always)]
    fn store(&mut self, index: LocalVarRef, val: i32) { self.local_vars[index as usize] = val; }

    #[inline(always)]
    fn store2(&mut self, index: LocalVarRef, val: [i32; 2]) {
        self.store(index, val[0]);
        self.store(index + 1, val[1]);
    }

    #[inline(always)]
    fn load(&self, index: LocalVarRef) -> i32 { self.local_vars[index as usize] }

    #[inline(always)]
    fn load2(&mut self, index: LocalVarRef) -> [i32; 2] { [self.load(index), self.load(index + 1)] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::max;

    macro_rules! arg1 { ($val: expr) => {{vec![unsafe {mem::transmute::<_, i32>($val)}]}} }
    macro_rules! arg2 { ($val: expr) => {{unsafe {mem::transmute::<_, [i32; 2]>($val)}.to_vec()}} }

    const TEST_CLASS: &'static str = "com/mackie/rustyjvm/TestVM";

    fn run(method: &str, native_calls: Vec<(&str, Vec<i32>)>) {
        let classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        let mut vm = VM::new(classloader);
        let mut start_frame = Frame::dummy_frame(0);
        vm.invoke_method(TEST_CLASS, method, "()V", &mut start_frame);
        vm.run(start_frame);

        for index in 0..max(native_calls.len(), vm.native_calls.len()) {
            if index >= native_calls.len() {
                println!("[{}] Additional Calls: {:?}",
                         method,
                         &vm.native_calls[index..]);
                panic!("FAIL");
            }
            if index >= vm.native_calls.len() {
                println!("[{}] Missing Calls: {:?}", method, &native_calls[index..]);
                panic!("FAIL");
            }
            assert_eq!((native_calls[index].0, &native_calls[index].1),
                       (vm.native_calls[index].0.as_str(), &vm.native_calls[index].2),
                       "index: {}",
                       index);
        }
    }

    #[test]
    fn print_class() {
        let mut classloader = ClassLoader::new(super::super::CLASSFILE_DIR);
        let class = classloader.load_class(TEST_CLASS).unwrap();
        for method in class.methods() {
            println!("Method {} {}:", method.descriptor(), method.name());
            match method.code() {
                Some(c) => {
                    for instr in c.code() {
                        println!("  {:?}", instr);
                    }
                }
                None => println!("  No code!"),
            }
            println!();
        }
    }

    #[test]
    fn simple() { run("simple", vec![("nativeInt", vec![1])]); }

    #[test]
    fn invoke() {
        run("invoke",
            vec![("nativeInt", arg1!(100)),
                 ("nativeInt", arg1!(200)),
                 ("nativeLong", arg2!(11i64)),
                 ("nativeInt", arg1!(300)),
                 ("nativeLong", arg2!(111i64)),
                 ("nativeLong", arg2!(111i64)),

                 ("nativeInt", arg1!(100)),
                 ("nativeInt", arg1!(200)),
                 ("nativeLong", arg2!(11i64)),
                 ("nativeInt", arg1!(300)),
                 ("nativeLong", arg2!(111i64)),
                 ("nativeLong", arg2!(111i64)),

                 ("nativeInt", arg1!(200)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(101i64)),

                 ("nativeInt", arg1!(1000)),
                 ("nativeInt", arg1!(2000)),
                 ("nativeLong", arg2!(11i64)),
                 ("nativeInt", arg1!(3000)),
                 ("nativeLong", arg2!(111i64)),
                 ("nativeLong", arg2!(111i64)),

                 ("nativeInt", arg1!(1000)),
                 ("nativeInt", arg1!(2000)),
                 ("nativeLong", arg2!(11i64)),
                 ("nativeInt", arg1!(3000)),
                 ("nativeLong", arg2!(111i64)),
                 ("nativeLong", arg2!(111i64)),

                 ("nativeInt", arg1!(2000)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(101i64)),

                 ("nativeInt", arg1!(1)),
                 ("nativeDouble", arg2!(1f64)),
                 ("nativeDouble", arg2!(2f64)),

                 ("nativeInt", arg1!(2)),
                 ("nativeDouble", arg2!(1f64)),
                 ("nativeDouble", arg2!(3f64)),

                 ("nativeInt", arg1!(2)),
                 ("nativeDouble", arg2!(1f64)),
                 ("nativeDouble", arg2!(3f64)),

                 ("nativeInt", arg1!(4)),
                 ("nativeDouble", arg2!(1f64)),
                 ("nativeDouble", arg2!(5f64)),

                 ("nativeInt", arg1!(3)),
                 ("nativeDouble", arg2!(1f64)),
                 ("nativeDouble", arg2!(4f64)),

                 ("nativeInt", arg1!(3)),
                 ("nativeDouble", arg2!(1f64)),
                 ("nativeDouble", arg2!(4f64)),

                 ("nativeInt", arg1!(42))]);
    }

    #[test]
    fn add() {
        run("add",
            vec![("nativeInt", arg1!(6)),
                 ("nativeInt", arg1!(6)),
                 ("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(0x7FFFFFFE)),
                 ("nativeInt", arg1!(0x7FFFFFFE)),
                 ("nativeLong", arg2!(6i64)),
                 ("nativeLong", arg2!(6i64)),
                 ("nativeLong", arg2!(0xFFFFFFFEi64)),
                 ("nativeLong", arg2!(0xFFFFFFFEi64)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(0x7FFFFFFFFFFFFFFEi64)),
                 ("nativeLong", arg2!(0x7FFFFFFFFFFFFFFEi64)),
                 ("nativeFloat", arg1!(2.1f32)),
                 ("nativeFloat", arg1!(2.1f32)),
                 ("nativeDouble", arg2!(2.1f64)),
                 ("nativeDouble", arg2!(2.1f64))]);
    }

    #[test]
    fn sub() {
        run("sub",
            vec![("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(1)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeFloat", arg1!(-1.9f32)),
                 ("nativeFloat", arg1!(-1.9f32)),
                 ("nativeDouble", arg2!(-1.9f64)),
                 ("nativeDouble", arg2!(-1.9f64))]);
    }

    #[test]
    fn mul() {
        run("mul",
            vec![("nativeInt", arg1!(8)),
                 ("nativeInt", arg1!(8)),
                 ("nativeInt", arg1!(4)),
                 ("nativeInt", arg1!(4)),
                 ("nativeLong", arg2!(8i64)),
                 ("nativeLong", arg2!(8i64)),
                 ("nativeLong", arg2!(0x400000010i64)),
                 ("nativeLong", arg2!(0x400000010i64)),
                 ("nativeLong", arg2!(4i64)),
                 ("nativeLong", arg2!(4i64)),
                 ("nativeFloat", arg1!(0.2f32)),
                 ("nativeFloat", arg1!(0.2f32)),
                 ("nativeDouble", arg2!(0.2f64)),
                 ("nativeDouble", arg2!(0.2f64))]);
    }

    #[test]
    fn div() {
        run("div",
            vec![("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(-1)),
                 ("nativeInt", arg1!(-1)),
                 ("nativeInt", arg1!(0x80000000u32)),
                 ("nativeInt", arg1!(0x80000000u32)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(-1i64)),
                 ("nativeLong", arg2!(-1i64)),
                 ("nativeLong", arg2!(0x8000000000000000u64)),
                 ("nativeLong", arg2!(0x8000000000000000u64)),
                 ("nativeFloat", arg1!(0.05f32)),
                 ("nativeFloat", arg1!(0.05f32)),
                 ("nativeDouble", arg2!(0.05f64)),
                 ("nativeDouble", arg2!(0.05f64))]);
    }

    #[test]
    fn rem() {
        run("rem",
            vec![("nativeInt", arg1!(2)),
                 ("nativeInt", arg1!(2)),
                 ("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(0)),
                 ("nativeInt", arg1!(0)),
                 ("nativeLong", arg2!(2i64)),
                 ("nativeLong", arg2!(2i64)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeFloat", arg1!(2.1f32 % 2.0f32)),
                 ("nativeFloat", arg1!(2.1f32 % 2.0f32)),
                 ("nativeDouble", arg2!(2.1f64 % 2.0f64)),
                 ("nativeDouble", arg2!(2.1f64 % 2.0f64))]);
    }

    #[test]
    fn neg() {
        run("neg",
            vec![("nativeInt", arg1!(-4)),
                 ("nativeInt", arg1!(-4)),
                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(0x80000000u32)),
                 ("nativeInt", arg1!(0x80000000u32)),
                 ("nativeLong", arg2!(-4i64)),
                 ("nativeLong", arg2!(-4i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(0x8000000000000000u64)),
                 ("nativeLong", arg2!(0x8000000000000000u64)),
                 ("nativeFloat", arg1!(-0.1f32)),
                 ("nativeFloat", arg1!(-0.1f32)),
                 ("nativeDouble", arg2!(-0.1f64)),
                 ("nativeDouble", arg2!(-0.1f64))]);
    }

    #[test]
    fn shift() {
        run("shift",
            vec![("nativeInt", arg1!(0xF0)),
                 ("nativeInt", arg1!(0xF0)),
                 ("nativeInt", arg1!(0x1E)),
                 ("nativeInt", arg1!(0x1E)),
                 ("nativeInt", arg1!(0x80000000u32)),
                 ("nativeInt", arg1!(0x80000000u32)),
                 ("nativeInt", arg1!(0)),
                 ("nativeInt", arg1!(0)),
                 ("nativeLong", arg2!(0xF0i64)),
                 ("nativeLong", arg2!(0xF0i64)),
                 ("nativeLong", arg2!(0x1Ei64)),
                 ("nativeLong", arg2!(0x1Ei64)),
                 ("nativeLong", arg2!(0x8000000000000000u64)),
                 ("nativeLong", arg2!(0x8000000000000000u64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeLong", arg2!(0i64)),

                 ("nativeInt", arg1!(0xF)),
                 ("nativeInt", arg1!(0xF)),
                 ("nativeInt", arg1!(0x7F)),
                 ("nativeInt", arg1!(0x7F)),
                 ("nativeInt", arg1!(0xC0000000u32)),
                 ("nativeInt", arg1!(0xC0000000u32)),
                 ("nativeInt", arg1!(-1)),
                 ("nativeInt", arg1!(-1)),
                 ("nativeLong", arg2!(0xFi64)),
                 ("nativeLong", arg2!(0xFi64)),
                 ("nativeLong", arg2!(0x7Fi64)),
                 ("nativeLong", arg2!(0x7Fi64)),
                 ("nativeLong", arg2!(0xC000000000000000u64)),
                 ("nativeLong", arg2!(0xC000000000000000u64)),
                 ("nativeLong", arg2!(-1i64)),
                 ("nativeLong", arg2!(-1i64)),

                 ("nativeInt", arg1!(0xF)),
                 ("nativeInt", arg1!(0xF)),
                 ("nativeInt", arg1!(0x7F)),
                 ("nativeInt", arg1!(0x7F)),
                 ("nativeInt", arg1!(0x40000000u32)),
                 ("nativeInt", arg1!(0x40000000u32)),
                 ("nativeInt", arg1!(0x7FFFFFFF)),
                 ("nativeInt", arg1!(0x7FFFFFFF)),
                 ("nativeLong", arg2!(0xFi64)),
                 ("nativeLong", arg2!(0xFi64)),
                 ("nativeLong", arg2!(0x7Fi64)),
                 ("nativeLong", arg2!(0x7Fi64)),
                 ("nativeLong", arg2!(0x4000000000000000u64)),
                 ("nativeLong", arg2!(0x4000000000000000u64)),
                 ("nativeLong", arg2!(0x7FFFFFFFFFFFFFFFi64)),
                 ("nativeLong", arg2!(0x7FFFFFFFFFFFFFFFi64))]);
    }

    #[test]
    fn bitops() {
        run("bitops",
            vec![("nativeInt", arg1!(0b1000)),
                 ("nativeInt", arg1!(0b1000)),
                 ("nativeInt", arg1!(0b1110)),
                 ("nativeInt", arg1!(0b1110)),
                 ("nativeInt", arg1!(0b0110)),
                 ("nativeInt", arg1!(0b0110)),
                 ("nativeLong", arg2!(0b1000i64)),
                 ("nativeLong", arg2!(0b1000i64)),
                 ("nativeLong", arg2!(0b1110i64)),
                 ("nativeLong", arg2!(0b1110i64)),
                 ("nativeLong", arg2!(0b0110i64)),
                 ("nativeLong", arg2!(0b0110i64))]);
    }

    #[test]
    fn iinc() {
        run("iinc",
            vec![("nativeInt", arg1!(0x80000000u32)),
                 ("nativeInt", arg1!(0x7fffffff)),
                 ("nativeInt", arg1!(0x7ffffff0))]);
    }

    #[test]
    fn constants() {
        run("constants",
            vec![("nativeInt", arg1!(0)),
                 ("nativeInt", arg1!(1337)),
                 ("nativeInt", arg1!(0x4000000)),
                 ("nativeFloat", arg1!(0f32)),
                 ("nativeFloat", arg1!(1f32)),
                 ("nativeFloat", arg1!(2f32)),
                 ("nativeFloat", arg1!(1.337f32)),
                 ("nativeDouble", arg2!(0f64)),
                 ("nativeDouble", arg2!(1f64)),
                 ("nativeDouble", arg2!(1.337f64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(1337i64)),
                 ("nativeString", arg1!(0))]);
    }

    #[test]
    fn conversions() {
        run("conversions",
            vec![("nativeByte", arg1!(-1)),
                 ("nativeByte", arg1!(-1)),
                 ("nativeShort", arg1!(-1)),
                 ("nativeShort", arg1!(-1)),
                 ("nativeChar", arg1!(0xFFFF)),
                 ("nativeChar", arg1!(0xFFFF)),

                 ("nativeLong", arg2!(5i64)),
                 ("nativeLong", arg2!(5i64)),
                 ("nativeFloat", arg1!(5.0f32)),
                 ("nativeFloat", arg1!(5.0f32)),
                 ("nativeDouble", arg2!(5.0f64)),
                 ("nativeDouble", arg2!(5.0f64)),

                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(1)),
                 ("nativeFloat", arg1!(4294967297.0f32)),
                 ("nativeFloat", arg1!(4294967297.0f32)),
                 ("nativeDouble", arg2!(4294967297.0f64)),
                 ("nativeDouble", arg2!(4294967297.0f64)),

                 ("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(-2)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeDouble", arg2!(-2.0999999046325684f64)),
                 ("nativeDouble", arg2!(-2.0999999046325684f64)),

                 ("nativeInt", arg1!(-2)),
                 ("nativeInt", arg1!(-2)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeLong", arg2!(-2i64)),
                 ("nativeFloat", arg1!(-2.1f32)),
                 ("nativeFloat", arg1!(-2.1f32))]);
    }

    #[test]
    fn jumps() {
        run("jumps",
            vec![("nativeInt", arg1!(-10)),
                 ("nativeInt", arg1!(-9)),
                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(2)),
                 ("nativeInt", arg1!(4)),
                 ("nativeInt", arg1!(9)),
                 ("nativeInt", arg1!(10)),
                 ("nativeInt", arg1!(11)),
                 ("nativeInt", arg1!(12)),
                 ("nativeInt", arg1!(14)),
                 ("nativeBoolean", arg1!(0)),
                 ("nativeBoolean", arg1!(1)),
                 ("nativeBoolean", arg1!(1)),
                 ("nativeBoolean", arg1!(0)),
                 ("nativeBoolean", arg1!(1)),
                 ("nativeBoolean", arg1!(0)),
                 ("nativeBoolean", arg1!(0)),
                 ("nativeBoolean", arg1!(0)),
                 ("nativeBoolean", arg1!(0)),
                 ("nativeBoolean", arg1!(0)),
                 ("nativeBoolean", arg1!(0))]);
    }

    #[test]
    fn arrays() {
        run("arrays",
            vec![("nativeLong", arg2!(0i64)),
                 ("nativeLong", arg2!(5i64)),
                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(2)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeLong", arg2!(5i64)),
                 ("nativeInt", arg1!(1)),
                 ("nativeInt", arg1!(2)),

                 ("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(2i64)),

                 ("nativeInt", arg1!(2)),
                 ("nativeInt", arg1!(3)),
                 ("nativeInt", arg1!(2))]);
    }

    #[test]
    fn object() {
        run("object",
            vec![("nativeInt", arg1!(10)),
                 ("nativeLong", arg2!(2i64)),
                 ("nativeDouble", arg2!(20.0f64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeInt", arg1!(50)),

                 ("nativeInt", arg1!(20)),
                 ("nativeLong", arg2!(2i64)),
                 ("nativeDouble", arg2!(20.0f64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeInt", arg1!(50)),

                 ("nativeInt", arg1!(20)),
                 ("nativeLong", arg2!(24i64)),
                 ("nativeDouble", arg2!(20.0f64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeInt", arg1!(50)),

                 ("nativeInt", arg1!(20)),
                 ("nativeLong", arg2!(24i64)),
                 ("nativeDouble", arg2!(40.0f64)),
                 ("nativeLong", arg2!(0i64)),
                 ("nativeInt", arg1!(50)),

                 ("nativeInt", arg1!(20)),
                 ("nativeLong", arg2!(24i64)),
                 ("nativeDouble", arg2!(40.0f64)),
                 ("nativeLong", arg2!(2i64)),
                 ("nativeInt", arg1!(50)),

                 ("nativeInt", arg1!(20)),
                 ("nativeLong", arg2!(24i64)),
                 ("nativeDouble", arg2!(40.0f64)),
                 ("nativeLong", arg2!(2i64)),
                 ("nativeInt", arg1!(200))]);
    }

}
