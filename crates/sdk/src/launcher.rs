use crate::{OnceLoaded, SourceSettings, WindowMode};
use std::ffi;
use std::ffi::{CString, OsStr, OsString};
use std::os::unix::ffi::OsStringExt;

type LauncherMain = unsafe extern "C" fn(ffi::c_int, *const *const ffi::c_char);

#[derive(Debug, Default)]
pub struct Args {
    args: Vec<OsString>,
}

impl Args {
    pub fn push(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.args.push(arg.as_ref().into());
        self
    }

    /// # Safety
    ///
    /// `launcher_main` must be valid.
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

impl From<&SourceSettings> for Args {
    fn from(settings: &SourceSettings) -> Self {
        let mut args = Self::default();

        args.push("csgo_linux64");

        if !settings.no_vac {
            args.push("-steam");
        }

        args.push(settings.renderer.params().option);

        if let Some(max_fps) = &settings.max_fps {
            args.push("+fps_max").push(max_fps.to_string());
        }

        match &settings.once_loaded {
            OnceLoaded::ConnectTo(address) => {
                args.push("+connect").push(address.to_string());
            }
            OnceLoaded::LoadMap(map) => {
                args.push("+map").push(map);
            }
            _ => {}
        }

        match settings.window_mode {
            WindowMode::Windowed => {
                args.push("-windowed");
            }
            WindowMode::Fullscreen => {
                args.push("-fullscreen");
            }
            _ => {}
        }

        args
    }
}
