#![deny(warnings)]
#![feature(concat_bytes)]
#![feature(extern_types)]
#![feature(ptr_metadata)]
#![feature(slice_from_ptr_range)]
#![feature(strict_provenance)]
#![feature(tuple_trait)]

use {
    region::Protection,
    std::{mem, ptr},
};

pub use {
    crate::{
        library::Library,
        map::{Map, Maps, Permissions},
        traits::{FnPtr, Ptr},
    },
    iced_x86,
};

mod library;
mod map;
mod traits;

#[doc(hidden)]
pub mod app;

#[doc(hidden)]
pub mod macros;

pub mod assembly;

/// Moves `src` into the pointed `dst`, returning the previous `dst` value.
///
/// # Safety
///
/// See [`replace`](std::ptr::replace).
pub unsafe fn replace<T>(dst: *mut T, src: T) -> T {
    let permissions = Maps::permissions_of(dst as *const u8);

    if permissions.contains(Permissions::WRITE) {
        ptr::replace(dst, src)
    } else {
        let _guard = region::protect_with_handle(
            dst as *const u8,
            mem::size_of::<T>(),
            Protection::READ_WRITE_EXECUTE,
        );

        ptr::replace(dst, src)
    }
}
