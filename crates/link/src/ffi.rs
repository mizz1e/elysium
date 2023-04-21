use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[inline]
unsafe fn cstr_inner(bytes: &[u8]) -> Cow<'_, CStr> {
    CStr::from_bytes_until_nul(bytes)
        .ok()
        .map(Cow::Borrowed)
        .or_else(|| CString::new(bytes).ok().map(Cow::Owned))
        .unwrap_unchecked()
}

#[inline]
pub fn cstr(bytes: &[u8]) -> Cow<'_, CStr> {
    unsafe { cstr_inner(bytes) }
}

#[inline]
pub fn with_cstr_os_str<S, F, T>(string: S, callback: F) -> T
where
    S: AsRef<OsStr>,
    F: FnOnce(Cow<'_, CStr>) -> T,
{
    let bytes = string.as_ref().as_bytes();
    let cstr = cstr(bytes);

    callback(cstr)
}

#[inline]
pub fn with_cstr_path<P, F, T>(path: P, callback: F) -> T
where
    P: AsRef<Path>,
    F: FnOnce(&Path, Cow<'_, CStr>) -> T,
{
    let path = path.as_ref();

    with_cstr_os_str(path, |cstr| callback(path, cstr))
}
