use crate::{ffi, module, query_address, Error, Module, SymbolAddress};
use std::ffi::OsStr;
use std::mem::ManuallyDrop;
use std::path::Path;
use std::{ops, ptr};

const FLAGS: libc::c_int = libc::RTLD_LOCAL | libc::RTLD_NOW;

#[derive(Debug)]
pub struct WithHandle {
    handle: *mut libc::c_void,
    module: Module,
}

impl WithHandle {
    #[inline]
    pub fn into_raw_parts(self) -> (*mut libc::c_void, Module) {
        let this = ManuallyDrop::new(self);

        unsafe {
            let handle = ptr::read(&this.handle);
            let module = ptr::read(&this.module);

            (handle, module)
        }
    }

    #[inline]
    pub unsafe fn from_raw_parts(handle: *mut libc::c_void, module: Module) -> Self {
        Self { handle, module }
    }

    #[inline]
    pub fn symbol<S>(&self, symbol: S) -> Result<SymbolAddress, Error>
    where
        S: AsRef<OsStr>,
    {
        ffi::with_cstr_os_str(symbol, |symbol| unsafe {
            symbol_inner(self.handle, symbol.as_ptr())
        })
    }
}

impl Drop for WithHandle {
    #[inline]
    fn drop(&mut self) {
        let _result = unsafe { close_inner(self.handle) };
    }
}

impl ops::Deref for WithHandle {
    type Target = Module;

    #[inline]
    fn deref(&self) -> &Module {
        &self.module
    }
}

#[inline]
unsafe fn open_inner(
    path: *const libc::c_char,
    flags: libc::c_int,
) -> Result<*mut libc::c_void, Error> {
    let handle = libc::dlopen(path, flags);

    if handle.is_null() {
        Err(Error::last_os_error().expect("os error"))
    } else {
        Ok(handle)
    }
}

#[inline]
unsafe fn symbol_inner(
    handle: *mut libc::c_void,
    symbol: *const libc::c_char,
) -> Result<SymbolAddress, Error> {
    let address = libc::dlsym(handle, symbol);

    if let Some(error) = Error::last_os_error() {
        Err(error)
    } else {
        // SAFETY: the addreas returned by dlsym should be valid to query.
        let address = query_address(address).unwrap_unchecked();
        let module = address.module;
        // SAFETY: the address queried is returned from a symbol lookup.
        let symbol = address.symbol.unwrap_unchecked();
        let address = SymbolAddress { module, symbol };

        Ok(address)
    }
}

#[inline]
unsafe fn close_inner(handle: *mut libc::c_void) -> Result<(), Error> {
    let result = libc::dlclose(handle);

    if result != 0 {
        Err(Error::last_os_error().expect("os error"))
    } else {
        Ok(())
    }
}

/// Attempt to load a module.
#[inline]
pub unsafe fn load_module<P>(path: P) -> Result<WithHandle, Error>
where
    P: AsRef<Path>,
{
    ffi::with_cstr_path(path, |path, cstr| unsafe {
        let file_name = path.file_name().expect("file_name");
        let handle = open_inner(cstr.as_ptr(), FLAGS)?;
        let mut current_module = Err(Error::NoModule);

        module::iterate_modules(|module| {
            if module.path.ends_with(file_name) {
                current_module = Ok(module);
            }
        });

        let module = current_module?;

        Ok(WithHandle::from_raw_parts(handle, module))
    })
}
