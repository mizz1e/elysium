use crate::{Maps, Ptr};
use std::{
    ffi::{CStr, OsStr},
    io,
    path::{Path, PathBuf},
};

/// A shared library.
pub struct Library {
    module: libloading::Library,
    path: PathBuf,
}

impl Library {
    /// Open a shared library.
    ///
    /// # Safety
    ///
    /// Caller must ensure the shared library to be loaded has safe initialization, and
    /// finalization routines, which are executed upon load, and unload, respectively.
    pub unsafe fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().canonicalize()?;

        libloading::Library::new(&path)
            .map(|module| Self { module, path })
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    /// Query the shared library for a symbol.
    pub fn get<T: Ptr>(&self, symbol: &CStr) -> io::Result<T> {
        unsafe { self.module.get::<T>(symbol.to_bytes_with_nul()) }
            .map(|symbol| unsafe { symbol.into_raw().into_raw().transmute() })
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    /// Returns the canonical path to this shared library.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the file name of this shared library.
    #[inline]
    pub fn file_name(&self) -> &OsStr {
        // SAFETY: Only senario I can think of would be you somehow loaded `/` as a shared library.
        unsafe { self.path.file_name().unwrap_unchecked() }
    }

    /// Returns memory maps referring to this shared library.
    pub fn maps(&self) -> io::Result<Maps> {
        let maps = Maps::current()?
            .into_iter()
            .filter(|map| {
                map.path()
                    .map(|path| path == self.path())
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>();

        Ok(Maps { maps })
    }
}
