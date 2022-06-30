#![feature(pointer_byte_offsets)]

//! Convenience wrapper around `elysium_dl::Library` for SDL methods.

use elysium_dl::Library;
use std::{fmt, ptr};

const LIBRARY: &str = "libSDL2-2.0.so.0\0";
const SWAP_WINDOW: &str = "SDL_GL_SwapWindow\0";
const POLL_EVENT: &str = "SDL_PollEvent\0";

/// The SDL library.
pub struct Sdl {
    library: Library,
}

impl Sdl {
    /// Load SDL, specifically `libSDL2-2.0.so.0`.
    #[inline]
    pub fn open() -> Option<Self> {
        let library = Library::open(LIBRARY)?;

        Some(Self { library })
    }

    #[inline]
    pub unsafe fn swap_window(&self) -> *const u8 {
        let address = match self.library.symbol(SWAP_WINDOW) {
            Some(symbol) => symbol.as_ptr().cast(),
            None => return ptr::null(),
        };

        elysium_mem::next_abs_addr(address)
    }

    /// 
    #[inline]
    pub unsafe fn poll_event(&self) -> *const u8 {
        let address = match self.library.symbol(POLL_EVENT) {
            Some(symbol) => symbol.as_ptr().cast(),
            None => return ptr::null(),
        };

        elysium_mem::next_abs_addr(address)
    }
}

impl fmt::Debug for Sdl {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.library, fmt)
    }
}
