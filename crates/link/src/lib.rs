#![feature(strict_provenance)]

use std::path::Path;

pub use address::{query_address, Address, Symbol, SymbolAddress};
pub use error::Error;
pub use module::{iterate_modules, Module};
pub use with_handle::{load_module, WithHandle};

mod address;
mod error;
mod ffi;
mod module;
mod with_handle;

/// Determine whether a module is loaded or not.
#[inline]
pub fn is_module_loaded<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut loaded = false;

    iterate_modules(|module| loaded |= module.path.ends_with(path));

    loaded
}
