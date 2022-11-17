use std::ffi;
use thiserror::Error;

/// An error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("interface not found: {0:?}")]
    Interface(Box<str>),

    #[error("failed to load module: {0}")]
    Module(#[from] libloading::Error),

    #[error("invalid C string: {0}")]
    CString(#[from] ffi::NulError),
}
