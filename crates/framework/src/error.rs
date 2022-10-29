use std::{error, ffi, fmt};

#[derive(Debug)]
pub enum Error {
    Interface,
    Link,
    Module(libloading::Error),
    NulStr(ffi::NulError),
    UnknownModule,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Interface => write!(fmt, "failed to construct an interface"),
            Error::Link => write!(fmt, "failed to link interfaces"),
            Error::Module(error) => write!(fmt, "failed to load module: {error}"),
            Error::NulStr(error) => write!(fmt, "string couldnt be made into a C string: {error}"),
            Error::UnknownModule => write!(fmt, "unknown module to construct an interface from"),
        }
    }
}
