use elysium_ptr::PtrMut;
use libloading::Library;
use std::ffi::CString;
use std::marker::PhantomPinned;
use std::mem::MaybeUninit;
use std::{ffi, mem};

use super::Error;

type NewInterface<'a, T> =
    unsafe extern "C" fn(name: *const ffi::c_char, result: &mut i32) -> MaybeUninit<PtrMut<'a, T>>;

/// An application module.
pub struct Module {
    new_interface: NewInterface<'static, ()>,

    #[allow(dead_code)]
    handle: Library,

    // It isn't particularily clear, but `new_interface` refers to a location within the module
    // `handle` points to. Dropping `handle` may invalidate that location (modules are
    // reference-counted). So, pretend this is a self-refferential structure.
    _pinned: PhantomPinned,
}

impl Module {
    /// Load a module.
    ///
    /// # Safety
    ///
    /// When loaded, initialization routines of a module are executed, which of course, may have
    /// unexpected side-effects.
    pub unsafe fn load(name: &str) -> Result<Module, Error> {
        let handle = libloading::Library::new(name).map_err(Error::Module)?;
        let new_interface = *handle.get(b"CreateInterface\0").map_err(Error::Module)?;

        Ok(Self {
            new_interface,
            handle,
            _pinned: PhantomPinned,
        })
    }

    /// Construct an interface from the specified module.
    ///
    /// # Safety
    ///
    /// Constructing certain interfaces may produce unexpected side-effects.
    pub unsafe fn new_interface<'a, T>(&mut self, name: &str) -> Result<PtrMut<'a, T>, Error> {
        let name = CString::new(name).map_err(Error::NulStr)?;
        let new_interface: NewInterface<'a, T> = mem::transmute(self.new_interface);

        let mut result = 0;
        let interface = (new_interface)(name.as_ptr(), &mut result);

        if result == 0 {
            Ok(interface.assume_init())
        } else {
            Err(Error::Interface)
        }
    }
}
