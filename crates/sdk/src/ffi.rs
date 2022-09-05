use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::slice;

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
pub fn with_cstr_os_str<S, C, T>(string: S, callback: C) -> T
where
    S: AsRef<OsStr>,
    C: FnOnce(Cow<'_, CStr>) -> T,
{
    let bytes = string.as_ref().as_bytes();
    let cstr = cstr(bytes);

    callback(cstr)
}

#[inline]
pub fn with_cstr_path<P, C, T>(path: P, callback: C) -> T
where
    P: AsRef<Path>,
    C: FnOnce(Cow<'_, CStr>) -> T,
{
    with_cstr_os_str(path.as_ref(), callback)
}

#[inline]
pub unsafe fn slice_from_i32<'a, T>(data: *const T, len: i32) -> &'a [T] {
    slice::from_raw_parts(data, len as usize)
}

#[inline]
pub const fn const_cstr(string: &str) -> &CStr {
    unsafe { CStr::from_bytes_with_nul_unchecked(string.as_bytes()) }
}

#[inline]
pub const fn const_cstr_opt(opt: Option<&str>) -> Option<&CStr> {
    match opt {
        Some(string) => Some(const_cstr(string)),
        None => None,
    }
}
