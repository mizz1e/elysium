use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    /// Architecture of a module was invalid.
    InvalidArchitecture(Box<OsStr>),
    /// A required module was not found.
    MissingDependency(Box<Path>),
    /// Module not found.
    NotFound(Box<Path>),
    /// Dynamic loading is not supported.
    NotSupported,
    /// No module associated.
    NoModule,
    /// A required symbol was not found.
    UndefinedSymbol(Box<Path>, Box<OsStr>, Option<Box<OsStr>>),
    /// Unknown error occured.
    Unknown(Box<OsStr>),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidArchitecture(arch) => {
                write!(fmt, "invalid architecture: {arch:?}")
            }
            Error::MissingDependency(dependency) => {
                write!(fmt, "missing dependency: {dependency:?}")
            }
            Error::NotFound(module) => write!(fmt, "module not found: {module:?}"),
            Error::NotSupported => write!(fmt, "dynamic loading is not supported"),
            Error::NoModule => write!(fmt, "no module aasociated"),
            Error::UndefinedSymbol(module, symbol, version) => {
                write!(fmt, "undefined symbol: {symbol:?} ")?;

                if let Some(version) = version {
                    write!(fmt, "({version:?}) ")?;
                }

                write!(fmt, "in {module:?}")?;

                Ok(())
            }
            Error::Unknown(error) => write!(fmt, "unknown: {error:?}"),
        }
    }
}

#[inline]
fn last_error<F, T>(f: F) -> Option<T>
where
    F: FnOnce(&[u8]) -> Option<T>,
{
    let error = unsafe { libc::dlerror() };

    if error.is_null() {
        None
    } else {
        let bytes = unsafe { CStr::from_ptr(error).to_bytes() };

        f(bytes)
    }
}

const NOT_SUPPORTED: &[u8] = b"Dynamic loading not supported";
const NOT_FOUND: &[u8] = b": cannot open shared object file: No such file or directory";
const UNDEFINED_SYMBOL: &[u8] = b": undefined symbol: ";
const SYMBOL_VERSION: &[u8] = b", version ";

#[inline]
fn split_once<'a>(bytes: &'a [u8], needle: &[u8]) -> Option<(&'a [u8], &'a [u8])> {
    memchr::memmem::find(bytes, needle).map(|offset| {
        let (left, right) = bytes.split_at(offset);
        let right = unsafe { right.get_unchecked(needle.len()..) };

        (left, right)
    })
}

#[inline]
fn os_str(bytes: &[u8]) -> &OsStr {
    OsStr::from_bytes(bytes)
}

#[inline]
fn boxed_os_str(bytes: &[u8]) -> Box<OsStr> {
    let string = os_str(bytes);

    Box::from(string)
}

#[inline]
fn boxed_path(bytes: &[u8]) -> Box<Path> {
    let string = os_str(bytes);
    let path = Path::new(string);

    Box::from(path)
}

#[inline]
fn map_internal_error(bytes: &[u8]) -> Option<Error> {
    //println!("{:?}", std::str::from_utf8(bytes));

    if bytes == NOT_SUPPORTED {
        return Some(Error::NotSupported);
    }

    if let Some(bytes) = bytes.strip_suffix(NOT_FOUND) {
        let path = boxed_path(bytes);

        return Some(Error::NotFound(path));
    }

    if let Some((path, rest)) = split_once(bytes, UNDEFINED_SYMBOL) {
        if let Some((symbol, version)) = split_once(rest, SYMBOL_VERSION) {
            let path = boxed_path(path);
            let symbol = boxed_os_str(symbol);
            let version = boxed_os_str(version);
            let version = Some(version);

            return Some(Error::UndefinedSymbol(path, symbol, version));
        } else {
            let path = boxed_path(path);
            let symbol = boxed_os_str(rest);
            let version = None;

            return Some(Error::UndefinedSymbol(path, symbol, version));
        }
    }

    None
}

impl Error {
    #[inline]
    pub fn last_os_error() -> Option<Self> {
        last_error(map_internal_error)
    }
}
