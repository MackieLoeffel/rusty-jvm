use classfile_parser::method_info::{PUBLIC, STATIC};
use class_loader::ClassLoader;

pub struct VM {
    classloader: ClassLoader,
    // stack: Vec<u8>,
    // call_frames: Vec<>
}

impl VM {
    pub fn new(loader: ClassLoader) -> VM { VM { classloader: loader } }

    // TODO think about using a real error type here
    pub fn run(&mut self, class: &str, _args: &[&str]) -> Result<(), String> {
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

        Ok(())
    }

    pub fn invoke_method(&mut self, class_name: &str, method: &str) {
        // these unwraps should be checked in the linking stage
        let method = self.classloader.load_class(class_name).unwrap().method_by_name(method).unwrap();

        // TODO ???
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(class: &str, method: &str) {
        let classloader = ClassLoader::new("./assets");
        let mut vm = VM::new(classloader);
        vm.invoke_method(class, method);
    }

    #[test]
    fn simple() { run("TestVM", "simple"); }

}
