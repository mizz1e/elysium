use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::{env, mem, ptr, slice};

/// Module information.
#[derive(Debug)]
pub struct Module {
    /// The address of this module.
    pub address: *const u8,

    /// Does this module contain TLS?
    pub has_tls: bool,

    /// The length (in bytes) of this module.
    pub len: usize,

    /// Path to this module.
    pub path: Box<Path>,
}

/// Callback data provided to `dl_iterate_phdr`.
struct Callback {
    pub callback: &'static mut dyn FnMut(Module),
}

impl Callback {
    /// Construct callback.
    ///
    /// # Safety
    ///
    /// Caller must ensure `callback` lives long enough.
    #[inline]
    pub unsafe fn new(callback: &mut dyn FnMut(Module)) -> Self {
        let callback = forge_static(callback);

        Self { callback }
    }

    /// Cast a reference to the callback to a void pointer.
    #[inline]
    pub fn as_raw(self: &mut Self) -> *mut libc::c_void {
        self as *mut Self as *mut libc::c_void
    }

    /// Obtain the callback from a void pointer.
    ///
    /// # Safety
    ///
    /// Caller must ensure `data` truly refers to a `Callback` structure.
    #[inline]
    pub unsafe fn from_raw<'a>(data: *mut libc::c_void) -> &'a mut Self {
        &mut *data.cast()
    }

    /// Callback passed to `dl_iterate_phdr`.
    #[inline]
    unsafe extern "C" fn callback(
        info: *mut libc::dl_phdr_info,
        size: libc::size_t,
        data: *mut libc::c_void,
    ) -> libc::c_int {
        callback_inner(info, size, data);

        0
    }
}

impl Module {
    #[inline]
    pub unsafe fn bytes(&self) -> &[u8] {
        slice::from_raw_parts(self.address, self.len)
    }

    #[inline]
    pub unsafe fn bytes_mut(&self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.address as *mut u8, self.len)
    }
}

/// Forge a static lifetime for the callback provided to `dl_iterate_phdr`.
#[inline]
unsafe fn forge_static(callback: &mut dyn FnMut(Module)) -> &'static mut dyn FnMut(Module) {
    mem::transmute(callback)
}

/// Obtains the module path from the `dlpi_name` pointer.
#[cfg(target_env = "gnu")]
#[inline]
pub(crate) unsafe fn module_path(path: *const libc::c_char) -> Option<Box<Path>> {
    let path = CStr::from_ptr(path).to_bytes();
    let os_str = OsStr::from_bytes(path);
    let path = Path::new(os_str);

    // glibc returns an empty string for the current module
    if os_str.is_empty() {
        Some(Box::from(env::current_exe().expect("current_exe")))
    // glibc returns "linux-vdso.so.1" for the vDSO
    } else if os_str == "linux-vdso.so.1" {
        None
    } else {
        let path = path.canonicalize().ok()?;

        Some(Box::from(path))
    }
}

/// Obtains the module path from the `dlpi_name` pointer.
#[cfg(target_env = "musl")]
#[inline]
pub(crate) unsafe fn module_path(path: *const libc::c_char) -> Option<Box<Path>> {
    let path = CStr::from_ptr(path).to_bytes();
    let os_str = OsStr::from_bytes(path);
    let path = Path::new(os_str);

    // musl returns "/proc/self/exe" for the current module
    if os_str == "/proc/self/exe" {
        Some(Box::from(env::current_exe().expect("current_exe")))
    // musl returns an empty string for the vDSO
    } else if os_str.is_empty() {
        None
    } else {
        let path = path.canonicalize().ok()?;

        Some(Box::from(path))
    }
}

/// Sanitize/map `dl_iterate_phdr` information.
#[inline]
unsafe fn callback_inner(
    info: *mut libc::dl_phdr_info,
    _size: libc::size_t,
    data: *mut libc::c_void,
) -> Option<()> {
    let callback = Callback::from_raw(data);
    let info = info.as_ref()?;

    // base module address
    let address: *const u8 = ptr::with_exposed_provenance(info.dlpi_addr as usize);

    // module length in bytes
    let headers = slice::from_raw_parts(info.dlpi_phdr, info.dlpi_phnum as usize);
    let len = headers
        .iter()
        .map(|header| header.p_memsz)
        .max()
        .unwrap_or_default() as usize;

    // returns None when the module is a vDSO, skip it
    let path = module_path(info.dlpi_name)?;

    // if this module has tls
    let has_tls = info.dlpi_tls_modid == 0 || info.dlpi_tls_data.is_null();

    let info = Module {
        address,
        has_tls,
        len,
        path,
    };

    (callback.callback)(info);

    Some(())
}

#[inline]
fn iterate_modules_inner(callback: &mut dyn FnMut(Module)) {
    // SAFETY: `callback`'s lifetime is valid for the duration of this function.
    let mut callback = unsafe { Callback::new(callback) };

    unsafe {
        libc::dl_iterate_phdr(Some(Callback::callback), Callback::as_raw(&mut callback));
    }
}

/// Iterate currently loaded modules.
#[inline]
pub fn iterate_modules(mut callback: impl FnMut(Module)) {
    iterate_modules_inner(&mut callback);
}
