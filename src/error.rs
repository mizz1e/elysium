use std::{ffi, fmt};

#[derive(Debug)]
pub enum Error {
    NoCsgo,
    NoDisplay,
    LoadLauncher(link::Error),
    FindMain(link::Error),
    InvalidArgs(ffi::NulError),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoCsgo => write!(fmt, "unable to find CSGO, do you even have steam?"),
            Error::NoDisplay => write!(fmt, "no DISPLAY"),
            Error::LoadLauncher(error) => write!(fmt, "unable to load launcher: {error}"),
            Error::FindMain(error) => write!(fmt, "unable to find main: {error}"),
            Error::InvalidArgs(error) => write!(fmt, "invalid args: {error}"),
        }
    }
}
