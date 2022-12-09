use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("ELF error: {0}")]
    Elf(#[from] goblin::error::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Cannot query information about the vDSO")]
    Vdso,
}
