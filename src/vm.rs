use classfile_parser::method_info::{PUBLIC, STATIC, NATIVE};
use class_loader::ClassLoader;
use instruction::{Instruction, LocalVarRef, CodeAddress};
use instruction::Instruction::*;
use instruction::Type::*;

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
        // TODO push args on the stack
        self.invoke_method(&class_name,
                           "main",
                           "([Ljava/lang/String;)V",
                           &mut Frame::dummy_frame(0));

        self.run();
        Ok(())
    }

    pub fn invoke_method(&mut self, class_name: &str, method: &str, descriptor: &str, calling_frame: &mut Frame) {
        // these unwraps should be checked in the linking stage
        let method = self.classloader.load_class(class_name).unwrap().method_by_signature(method, descriptor).unwrap();

        let args = &calling_frame.stack[calling_frame.sp - method.words_for_params()..calling_frame.sp];
        calling_frame.sp -= method.words_for_params();

        if method.access_flags().contains(NATIVE) {
            self.native_calls.push((method.name().to_owned(), method.descriptor().to_owned(), args.to_vec()));
            // TODO real handling of call
            return;
        }

        let code = method.code().expect("Method must have code");
        println!("Code: {:?}", code);

        let mut local_vars = Vec::with_capacity(code.max_locals());
        local_vars.resize(code.max_locals(), 0);
        local_vars[..args.len()].copy_from_slice(args);
        let mut stack = Vec::with_capacity(code.max_stack());
        stack.resize(code.max_stack(), 0);
        self.frames.push(Frame {
            ip: 0,
            sp: 0,
            local_vars: local_vars,
            stack: stack,
            code: code.code().clone(),
        });
    }

    fn run(&mut self) {
        let mut frame = self.frames.pop().expect("No frame supplied for run");
        loop {
            match frame.next() {
                BIPUSH(i) => frame.push(i as i32),
                STORE(typ, idx) => {
                    if typ.is_double_sized() {
                        // TODO test
                        let v = frame.pop();
                        frame.store(idx + 1, v);
                    }
                    let v = frame.pop();
                    frame.store(idx, v);
                }
                LOAD(typ, idx) => {
                    let v = frame.load(idx);
                    frame.push(v);
                    if typ.is_double_sized() {
                        // TODO test
                        let v2 = frame.load(idx + 1);
                        frame.push(v2);
                    }
                }
                RETURN(o) => {
                    if self.frames.is_empty() {
                        return;
                    }
                    let mut old_frame = frame;
                    frame = self.frames.pop().unwrap();

                    if let Some(typ) = o {
                        let v = old_frame.pop();
                        frame.push(v);
                        if typ.is_double_sized() {
                            // TODO test
                            let v2 = old_frame.pop();
                            frame.push(v2);
                        }
                    }
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
    fn pop(&mut self) -> i32 {
        self.sp -= 1;
        self.stack[self.sp]
    }

    #[inline(always)]
    fn store(&mut self, index: LocalVarRef, val: i32) { self.local_vars[index as usize] = val; }

    #[inline(always)]
    fn load(&self, index: LocalVarRef) -> i32 { self.local_vars[index as usize] }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(class: &str, method: &str, native_calls: Vec<(String, String, Vec<i32>)>) {
        let classloader = ClassLoader::new("./assets");
        let mut vm = VM::new(classloader);
        vm.invoke_method(class, method, "()V", &mut Frame::dummy_frame(0));
        vm.run();

        assert_eq!(native_calls, vm.native_calls);
    }

    #[test]
    fn simple() { run("TestVM", "simple", vec![]); }

}
