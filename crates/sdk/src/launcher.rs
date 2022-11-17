use std::ffi;
use std::ffi::{CString, OsStr, OsString};
use std::os::unix::ffi::OsStringExt;

type LauncherMain = unsafe extern "C" fn(ffi::c_int, *const *const ffi::c_char);

#[derive(Debug, Default)]
pub struct Args {
    args: Vec<OsString>,
}

impl Args {
    #[inline]
    pub fn push(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.args.push(arg.as_ref().into());
        self
    }

    /// # Safety
    ///
    /// `launcher_main` must be valid.
    #[inline]
    pub unsafe fn exec(self, launcher_main: LauncherMain) {
        tracing::trace!("launch with args: {:?}", self.args);

        let args = self
            .args
            .into_iter()
            .map(OsStringExt::into_vec)
            .map(CString::new)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();
        let len = args.len();

        (launcher_main)(len as i32, args.as_ptr());
    }
}
