#![deny(warnings)]
#![feature(concat_bytes)]
#![feature(extern_types)]
#![feature(ptr_metadata)]
#![feature(slice_from_ptr_range)]
#![feature(strict_provenance)]
#![feature(tuple_trait)]

pub use crate::{
    library::Library,
    map::{Map, Maps, Permissions},
    traits::{FnPtr, Ptr},
};
pub use iced_x86;

mod library;
mod map;
mod traits;

#[doc(hidden)]
pub mod macros;

pub mod assembly;
