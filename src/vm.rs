use classfile_parser::method_info::{PUBLIC, STATIC, NATIVE};
use class_loader::ClassLoader;
use instruction::{Instruction, LocalVarRef};
use instruction::Instruction::*;
use instruction::Type::*;
use parsed_class::MethodRef;
use std::mem;
use std::cmp::max;
use std::ops::Mul;

// USE WITH CARE
macro_rules! conv { ($val: expr) => {{unsafe {mem::transmute($val)}}} }

pub struct VM {
    classloader: ClassLoader,
    frames: Vec<Frame>,
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
}

impl VM {
    pub fn new(loader: ClassLoader) -> VM {
        VM {
            native_calls: Vec::new(),
            classloader: loader,
            frames: Vec::new(),
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
        let mut start_frame = Frame::dummy_frame(0);
        // TODO push args on the stack
        self.invoke_method(&class_name,
                           "main",
                           "([Ljava/lang/String;)V",
                           &mut start_frame);

        self.run(start_frame);
        Ok(())
    }

    pub fn invoke_method(&mut self, class_name: &str, method: &str, descriptor: &str, calling_frame: &mut Frame) {
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
        };
        mem::swap(&mut new_frame, calling_frame);
        self.frames.push(new_frame);
    }

    pub fn invoke_method_ref(&mut self, method: &MethodRef, calling_frame: &mut Frame) {
        self.invoke_method(method.class(),
                           method.name(),
                           method.descriptor(),
                           calling_frame)
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
                t@_ => panic!("Operation {} is not implemented for typ {:?}", stringify!($op), t),
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
                t@_ => panic!("Operation {} is not implemented for typ {:?}", stringify!($op), t),
            }
        }});

        loop {
            match frame.next() {
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
                MUL(t @ Int) | MUL(t @ Long) => arith_int!(t, wrapping_mul),
                MUL(t) => arith_float!(t, mul),
                // MUL(Int) => {
                // let a: i32 = conv!(frame.pop());
                // let b: i32 = conv!(frame.pop());
                // frame.push(a.wrapping_mul(b));
                // }
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
                INVOKESTATIC(method) => {
                    self.invoke_method_ref(&method, &mut frame);
                }
                c @ _ => panic!("Not implemented Instruction {:?}", c),
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
        }
    }

    #[inline(always)]
    fn next(&mut self) -> Instruction {
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
    macro_rules! arg1 { ($val: expr) => {{vec![unsafe {mem::transmute::<_, i32>($val)}]}} }
    macro_rules! arg2 { ($val: expr) => {{unsafe {mem::transmute::<_, [i32; 2]>($val)}.to_vec()}} }
    use super::*;

    fn run(class: &str, method: &str, native_calls: Vec<(&str, Vec<i32>)>) {
        let classloader = ClassLoader::new("./assets");
        let mut vm = VM::new(classloader);
        let mut start_frame = Frame::dummy_frame(0);
        vm.invoke_method(class, method, "()V", &mut start_frame);
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
                       (vm.native_calls[index].0.as_str(), &vm.native_calls[index].2));
        }
    }

    #[test]
    fn print_class() {
        let mut classloader = ClassLoader::new("./assets");
        let class = classloader.load_class("TestVM").unwrap();
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
    fn simple() { run("TestVM", "simple", vec![("nativeInt", vec![1])]); }

    #[test]
    fn staticcall() {
        run("TestVM",
            "staticcall",
            vec![("nativeLong", arg2!(1i64)),
                 ("nativeLong", arg2!(2i64)),
                 ("nativeLong", arg2!(2i64))]);
    }

    #[test]
    fn mul() {
        run("TestVM",
            "mul",
            vec![("nativeInt", arg1!(8)),
                 ("nativeInt", arg1!(4)),
                 ("nativeLong", arg2!(8i64)),
                 ("nativeLong", arg2!(0x400000010i64)),
                 ("nativeLong", arg2!(4i64)),
                 ("nativeFloat", arg1!(0.2f32)),
                 ("nativeDouble", arg2!(0.2f64)),
            ]);
    }

    #[test]
    fn constants() {
        run("TestVM",
            "constants",
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
}
