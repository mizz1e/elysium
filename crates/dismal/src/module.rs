use crate::{Error, Pattern, Result};
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::{env, ffi, ptr, slice};

#[cfg(target_env = "gnu")]
pub unsafe fn determine_path(path: *const ffi::c_char) -> Result<PathBuf> {
    let bytes = CStr::from_ptr(path).to_bytes();

    // Empty string indicates the current process.
    if bytes.is_empty() {
        env::current_exe().map_err(Into::into)
    } else if bytes == b"linux-vdso.so.1" {
        Err(Error::Vdso)
    } else {
        Ok(OsStr::from_bytes(bytes).into())
    }
}

#[cfg(target_env = "musl")]
pub unsafe fn determine_path(path: *const ffi::c_char) -> Result<PathBuf> {
    let bytes = CStr::from_ptr(path).to_bytes();

    // Empty string indicates the vDSO.
    if bytes.is_empty() {
        Err(Error::Vdso)
    } else if bytes == b"/proc/self/exe" {
        env::current_exe().into()
    } else {
        Ok(OsStr::from_bytes(bytes).into())
    }
}

#[derive(Clone, Debug)]
pub struct Code {
    pub base_addr: usize,
    pub offset: usize,
    pub len: usize,
    pub name: String,
}

impl Code {
    pub unsafe fn memory(&self) -> &[u8] {
        let ptr = ptr::from_exposed_addr::<u8>(self.base_addr + self.offset);

        slice::from_raw_parts(ptr, self.len)
    }

    pub unsafe fn memory_mut(&mut self) -> &mut [u8] {
        let ptr = ptr::from_exposed_addr_mut::<u8>(self.base_addr + self.offset);

        slice::from_raw_parts_mut(ptr, self.len)
    }
}

#[derive(Debug)]
pub struct Module {
    pub path: PathBuf,
    pub code: Vec<Code>,
}

pub struct Search<'a> {
    pub bytes: &'a [u8],
    pub code: Code,
}

impl Module {
    pub unsafe fn search(&self, pattern: &Pattern) -> Option<Search<'_>> {
        for code in &self.code {
            if let Some(bytes) = pattern.find(code.memory()) {
                return Some(Search {
                    bytes,
                    code: code.clone(),
                });
            }
        }

        None
    }
}
