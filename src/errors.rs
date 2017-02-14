use std::error;
use std::fmt;
use std::io;
use parsed_class::FieldRef;

#[derive(Debug)]
pub enum ClassLoadingError {
    NoClassDefFound(Result<String, io::Error>),
    ClassFormatError(String),
    UnsupportedClassVersion,
    NoSuchFieldError(FieldRef),
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
                       err.as_ref().map(|o| o.to_owned()).map_err(|e| format!("{}", e)).unwrap_or_else(|e| e))
            }
            ClassLoadingError::ClassFormatError(ref err) => write!(f, "ClassFormatError: {}", err),
            ClassLoadingError::NoSuchFieldError(ref field) => write!(f, "NoSuchField: {:?}", field),
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
            ClassLoadingError::NoSuchFieldError(..) => "NoSuchFieldError",
            ClassLoadingError::UnsupportedClassVersion => "UnsupportedClassVersion",
            ClassLoadingError::IncompatibleClassChange => "IncompatibleClassChange",
            ClassLoadingError::ClassCircularity => "ClassCircularity",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ClassLoadingError::NoClassDefFound(ref err) => err.as_ref().err().map(|e| e as &error::Error),
            _ => None,
        }
    }
}
