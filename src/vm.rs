use classfile_parser::method_info::{PUBLIC, STATIC};
use class_loader::ClassLoader;
use instruction::Instruction;
use instruction::Instruction::*;

pub struct VM {
    classloader: ClassLoader,
    frames: Vec<Frame>,
}

pub struct Frame {
    // arrays are to big to create them on the stack
    // TODO think about using maybe Box<Frame> with arrays
    //  => benchmark
    code: Vec<Instruction>,
    ip: usize,
    sp: usize,
    local_vars: Vec<u32>,
    stack: Vec<u32>,
}


impl VM {
    pub fn new(loader: ClassLoader) -> VM {
        VM {
            classloader: loader,
            frames: Vec::new(),
        }
    }

    // TODO think about using a real error type here
    pub fn start(&mut self, class: &str, _args: &[&str]) -> Result<(), String> {
        let class_name;
        {
            let start_class = self.classloader.load_class(class).map_err(|err| format!("ClassLoadingError: {}", err))?;

            let main = match start_class.method_by_name("main") {
                Some(m) => m,
                None => return Err("No main method found!".to_owned()),
            };

            if main.access_flags() != PUBLIC | STATIC {
                return Err(format!("invalid access flags for main: {:?}", main.access_flags()));
            }

            if main.descriptor() != "([Ljava/lang/String;)V" {
                return Err(format!("signatur for main: {}", main.descriptor()));
            }
            class_name = start_class.name().to_owned();
        }
        // TODO push args on the stack
        self.invoke_method(&class_name, "main");

        self.run();
        Ok(())
    }

    pub fn invoke_method(&mut self, class_name: &str, method: &str) {
        // these unwraps should be checked in the linking stage
        let method = self.classloader.load_class(class_name).unwrap().method_by_name(method).unwrap();
        let code = method.code().expect("Method must have code");

        self.frames.push( Frame {
            ip: 0,
            sp: 0,
            local_vars: Vec::with_capacity(code.max_locals()),
            stack: Vec::with_capacity(code.max_stack()),
            code: code.code().clone(),
        });
    }

    fn run(&mut self) {
        let frame = self.frames.pop().expect("No frame supplied for run");
        loop {
            match frame.code[frame.ip].clone() {
                c@ _ => panic!("Not implemented Instruction {:?}", c),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(class: &str, method: &str) {
        let classloader = ClassLoader::new("./assets");
        let mut vm = VM::new(classloader);
        vm.invoke_method(class, method);
        vm.run();
    }

    #[test]
    fn simple() { run("TestVM", "simple"); }

}
