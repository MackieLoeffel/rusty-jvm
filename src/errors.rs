use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ClassLoadingError {
    NoClassDefFound(Option<io::Error>),
    ClassFormatError(String),
    UnsupportedClassVersion,
    #[allow(dead_code)]
    IncompatibleClassChange,
    #[allow(dead_code)]
    ClassCircularity,
}

impl fmt::Display for ClassLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClassLoadingError::NoClassDefFound(ref err) => {
                write!(f,
                       "NoClassDefFound: {}",
                       err.as_ref().map(|e| format!("{}", e)).unwrap_or("".to_owned()))
            }
            ClassLoadingError::ClassFormatError(ref err) => write!(f, "ClassFormatError: {}", err),
            ClassLoadingError::UnsupportedClassVersion => write!(f, "class version not supported"),
            ClassLoadingError::IncompatibleClassChange => write!(f, "IncompatibleClassChange"),
            ClassLoadingError::ClassCircularity => write!(f, "ClassCircularity"),
        }
    }
}

impl error::Error for ClassLoadingError {
    fn description(&self) -> &str {
        match *self {
            ClassLoadingError::NoClassDefFound(..) => "NoClassDefFound",
            ClassLoadingError::ClassFormatError(..) => "ClassFormatError",
            ClassLoadingError::UnsupportedClassVersion => "UnsupportedClassVersion",
            ClassLoadingError::IncompatibleClassChange => "IncompatibleClassChange",
            ClassLoadingError::ClassCircularity => "ClassCircularity",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ClassLoadingError::NoClassDefFound(ref err) => err.as_ref().map(|e| e as &error::Error),
            _ => None,
        }
    }
}
