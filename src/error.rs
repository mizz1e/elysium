use std::{error, ffi, fmt, io};

#[derive(Debug)]
pub enum Error {
    FindMain(link::Error),
    Framework(elysium_framework::Error),
    Io(io::Error),
    LoadLauncher(link::Error),
    InvalidArgs(ffi::NulError),
    NoCsgo,
    NoDisplay,
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FindMain(error) => write!(fmt, "unable to find main: {error}"),
            Error::Framework(error) => write!(fmt, "framework error: {error}"),
            Error::InvalidArgs(error) => write!(fmt, "invalid args: {error}"),
            Error::LoadLauncher(error) => write!(fmt, "unable to load launcher: {error}"),
            Error::Io(error) => write!(fmt, "{error}"),
            Error::NoCsgo => write!(fmt, "unable to find CSGO, do you even have steam?"),
            Error::NoDisplay => write!(fmt, "no DISPLAY"),
        }
    }
}

impl error::Error for Error {}

impl From<elysium_framework::Error> for Error {
    fn from(error: elysium_framework::Error) -> Error {
        Error::Framework(error)
    }
}
